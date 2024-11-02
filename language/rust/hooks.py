import shutil
from pathlib import Path
import json

SETTINGS_FILE = Path(".vscode/settings.json")


def on_start(rootdir, workdir, tmpdir, solver_file, cases_file):
    # テンプレートから作業ディレクトリに Cargo ファイルをコピー
    cargo_path = Path(workdir, "Cargo.toml")
    shutil.copy("language/rust/Cargo.toml", cargo_path)

    if SETTINGS_FILE.exists():
        with SETTINGS_FILE.open("r", encoding="utf-8") as f:
            settings = json.load(f)

        # VSCode が作業ディレクトリを Rust プロジェクトとして認識するよう
        # settings.json に追加
        linked_projects = settings.get("rust-analyzer.linkedProjects", [])
        cargo_abs_path = str(cargo_path.resolve())
        if cargo_abs_path not in linked_projects:
            linked_projects.append(cargo_abs_path)
        settings["rust-analyzer.linkedProjects"] = linked_projects

        with SETTINGS_FILE.open("w", encoding="utf-8") as f:
            json.dump(settings, f, indent=4)
            f.write("\n")


def on_exit(rootdir, workdir, tmpdir, solver_file, cases_file):
    # Rust プロジェクト関連ファイルを削除
    cargo_path = Path(workdir, "Cargo.toml")
    cargo_lock_path = Path(workdir, "Cargo.lock")
    target_path = Path(workdir, "target")
    if cargo_path.exists():
        cargo_path.unlink()
    if cargo_lock_path.exists():
        cargo_lock_path.unlink()
    if target_path.exists():
        shutil.rmtree(Path(workdir, "target"))

    if SETTINGS_FILE.exists():
        with SETTINGS_FILE.open("r", encoding="utf-8") as f:
            settings = json.load(f)

        # settings.json からプロジェクトパス（作業ディレクトリ）を削除
        linked_projects = settings.get("rust-analyzer.linkedProjects", [])
        cargo_abs_path = str(cargo_path.resolve())
        if cargo_abs_path in linked_projects:
            linked_projects = list(
                filter(lambda x: x != cargo_abs_path, linked_projects)
            )

        if linked_projects:
            settings["rust-analyzer.linkedProjects"] = linked_projects
        else:
            # 空になるならキーごと削除
            del settings["rust-analyzer.linkedProjects"]

        with SETTINGS_FILE.open("w", encoding="utf-8") as f:
            json.dump(settings, f, indent=4)
            f.write("\n")
