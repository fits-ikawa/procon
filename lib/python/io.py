
def astypes(line, *types):
    return tuple(map(lambda p: p[0](p[1]),
                     zip(types, line.strip().split(" "))))


def with_guards(height, width, guard):
    field = [guard * (width + 2)]
    field += [guard + input().strip() + guard
              for _ in range(height)]
    field += [guard * (width + 2)]
    return field


def map_with_guards(height, width, guard, mapper):
    field = [[guard] * (width + 2)]
    field += [[guard] + list(map(mapper, input().strip())) + [guard]
              for _ in range(height)]
    field += [[guard] * (width + 2)]
    return field
