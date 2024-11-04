import os
import sys
import shutil
import re
import hashlib
import yaml
import tempfile
from pathlib import Path
import importlib.util

from invoke import task, run
from invoke.exceptions import UnexpectedExit, CommandTimedOut

from watchdog.observers import Observer
from watchdog.events import PatternMatchingEventHandler

TIMEOUT_SEC = 10


def load_global_config():
    config_path = Path("global.yml")
    if not config_path.exists():
        raise FileNotFoundError("Global configuration file 'global.yml' not found.")
    with open(config_path, "r") as f:
        return yaml.safe_load(f)


def load_language_config(language):
    config_path = Path(f"language/{language}.yml")
    if not config_path.exists():
        raise FileNotFoundError(
            f"Configuration file for language '{language}' not found."
        )
    with open(config_path, "r") as f:
        config = yaml.safe_load(f)

    return config


def do_test(
    basedir,
    tmpdir,
    language_config,
    solver_file,
    cases_file,
):
    solver = Path(basedir, solver_file)
    cases = Path(basedir, cases_file)

    # ビルドコマンドがあれば実行
    if "build_command" in language_config:
        build_cmd = language_config["build_command"].format(
            rootdir=Path(__file__).parent, workdir=basedir, tmpdir=tmpdir, solver=solver
        )
        try:
            run(build_cmd, echo=True)
        except UnexpectedExit:
            print("Build failed.")
            return

    # テストケースを読み込む
    blocks = re.split(r"\n{2,}", open(cases, "r").read())
    if len(blocks) == 0 or len(blocks) % 2 != 0:
        print("input-output pair mismatch\n")
        return

    print(solver)

    results = []

    # PICK ケースがあるか調べる
    pick = False
    for i in range(0, len(blocks), 2):
        feed = blocks[i].strip() + "\n"
        pick = pick or feed.upper().startswith("#PICK")

    for i in range(0, len(blocks), 2):
        # 入力
        feed = blocks[i].strip() + "\n"

        if pick and not feed.upper().startswith("#PICK"):
            # PICK が指定されたケース以外は飛ばす
            results.append("SK")
            continue

        elif feed.upper().startswith("#SKIPBELOW"):
            # 後続のテストケースをすべて飛ばす
            results.append("SK")
            break

        elif feed.upper().startswith("#SKIP"):
            # このテストケースを飛ばす
            results.append("SK")
            continue

        if pick:
            feed = re.sub(r"^#PICK\n", "", feed, flags=re.IGNORECASE | re.MULTILINE)

        # 解答
        answer = (
            re.sub(r"^#BLANK", "", blocks[i + 1].strip(), flags=re.IGNORECASE) + "\n"
        )

        print(f"==== case {(i // 2) + 1} ====")
        print(answer.strip())
        print("----------------")
        try:
            # スクリプト実行
            cmd = (language_config["run_command"] + " <<EOF\n{feed}EOF\n").format(
                rootdir=Path(__file__).parent,
                workdir=basedir,
                tmpdir=tmpdir,
                solver=solver,
                feed=feed,
            )
            result = run(cmd, timeout=TIMEOUT_SEC)
            if answer.strip().upper() == "#DONTCARE":
                # 解答なしケース
                results.append("DC")
            else:
                # 解答ありケースは stdout と比較
                results.append("AC" if result.stdout == answer else "WA")

        except UnexpectedExit:
            # solver の実行時エラーのとき
            # Invoke 側のスタックトレースを省くための握り潰し。
            # 他の異常終了も潰しそうだけど困った時に考える
            break

        except CommandTimedOut:
            # タイムアウトが発生したら以降のケースを中断する
            print(f"Execution timed out: over {TIMEOUT_SEC} seconds.")
            print("Test aborted.")
            break

        print()

    print(" ".join(results))
    print()


class AutoTestHandler(PatternMatchingEventHandler):
    def __init__(self, basedir, tmpdir, language_config, targets):
        super().__init__(targets)
        self.basedir = basedir
        self.tmpdir = tmpdir
        self.filehash = {}
        self.language_config = language_config
        self.targets = targets

    def on_modified(self, event):
        # ファイルの内容変更がある場合のみテストを実行する
        new_stamp = hashlib.sha1(open(event.src_path, "rb").read()).hexdigest()
        if old_stamp := self.filehash.get(event.src_path):
            if old_stamp == new_stamp:
                # 更新無し
                return

        self.filehash[event.src_path] = new_stamp

        msg = f"Modified on {event.src_path}"
        hr = ("=-" * (len(msg) // 2)) + "="
        print(f"\n{hr}\n{msg}\n")

        do_test(self.basedir, self.tmpdir, self.language_config, *self.targets)

        print(f'Watching "{self.basedir}" ...')
        print("cmd >> ", end="", flush=True)


@task
def procon(_, language, path, retry=False):
    basedir = Path(path)
    os.makedirs(basedir, exist_ok=True)

    language_config = load_language_config(language)
    global_config = load_global_config()

    if retry and "solver_retry_file" not in language_config:
        print(f"{language} doesn't support retry.")
        return

    # --retry オプションを付けるとソルバを別名のファイルにする。
    # 問題を解き直したい場合などに使用する。
    # TODO: 1 個の別名しかサポートしてないので、自動付番にしたい
    solver_file = (
        language_config["solver_file"]
        if not retry
        else language_config["solver_retry_file"]
    )
    cases_file = "cases.txt"

    solver = Path(basedir, solver_file)
    cases = Path(basedir, cases_file)

    solver.touch()
    cases.touch()

    with tempfile.TemporaryDirectory(dir="tmp") as tmpdir:
        if "extra_files" in language_config:
            # 言語別の追加ファイルをコピー
            for src, dst in language_config["extra_files"]:
                shutil.copy(src, dst.format(workdir=basedir, tmpdir=tmpdir))

        # VSCode でファイルを開く
        if global_config.get("open_in_vscode", False):
            run(f"code {solver}")
            run(f"code {cases}")

        watch(path, solver_file, cases_file, language_config, tmpdir)


def watch(path, solver_file, cases_file, language_config, tmpdir):
    basedir = Path(path)
    observer = Observer()
    observer.schedule(
        AutoTestHandler(basedir, tmpdir, language_config, [solver_file, cases_file]),
        basedir,
    )

    # 言語別の追加処理を実行（開始時のフック）
    hooks_module = None
    if "hooks" in language_config:
        hooks_path = language_config["hooks"]
        module_name = Path(hooks_path).stem

        spec = importlib.util.spec_from_file_location(module_name, hooks_path)
        hooks_module = importlib.util.module_from_spec(spec)
        sys.modules[module_name] = hooks_module
        spec.loader.exec_module(hooks_module)

        on_start = getattr(hooks_module, "on_start", None)
        if on_start is not None:
            on_start(Path(__file__).parent, basedir, tmpdir, solver_file, cases_file)

    print(f'Start watching "{basedir}" ...')
    observer.start()

    solver = Path(basedir, solver_file)
    gencase = Path(basedir, "gencase.py")
    bigcase = Path(basedir, "bigcase.txt")

    try:
        while cmd := input("cmd >> "):
            try:
                if cmd == "g":
                    # テストケース生成スクリプト（Python）を実行する。
                    # 出力を cases.txt に手動でコピーする想定
                    gencase.touch()
                    run(f"python {gencase}", echo=True)

                elif cmd == "b":
                    # テストケース生成スクリプト（Python）を実行し、
                    # その出力をすぐにソルバに食わせる。
                    # 大規模データのテストに使う想定なので
                    # bigcase.txt というファイルに生成したケースを保存する
                    gencase.touch()
                    run(f"python {gencase} >{bigcase}", echo=True)

                    # ビルドコマンドがあれば実行
                    if "build_command" in language_config:
                        build_cmd = language_config["build_command"].format(
                            rootdir=Path(__file__).parent,
                            workdir=basedir,
                            tmpdir=tmpdir,
                            solver=solver,
                        )
                        try:
                            run(build_cmd, echo=True)
                        except UnexpectedExit:
                            print("Build failed.")
                            continue

                    run(
                        (
                            language_config["run_command"] + f" <{bigcase.resolve()}"
                        ).format(
                            rootdir=Path(__file__).parent,
                            workdir=basedir,
                            tmpdir=tmpdir,
                            solver=solver,
                        ),
                        echo=True,
                    )

                elif cmd == "q":
                    # 終了
                    break

            except UnexpectedExit:
                continue

    except KeyboardInterrupt:
        pass

    finally:
        print("Finishing...")
        observer.unschedule_all()
        observer.stop()
        observer.join()

        # 言語別の追加処理を実行（終了時のフック）
        if hooks_module is not None:
            on_exit = getattr(hooks_module, "on_exit", None)
            if on_exit is not None:
                on_exit(Path(__file__).parent, basedir, tmpdir, solver_file, cases_file)

        print("Done.")
