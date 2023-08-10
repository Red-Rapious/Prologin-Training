def state(d, m, n, positions):
    """
    :param d: distance entre les deux planètes
    :type d: int
    :param m: distance à laquelle Joseph s'est téléporté
    :type m: int
    :param n: nombre de trous noirs
    :type n: int
    :param positions: liste des positions des trous noirs
    :type positions: list[int]
    """
    # TODO Retourne l'état de Joseph Marchand. -1 s'il a été happé par un trou
    # noir, 0 s'il s'est perdu en chemin, 1 s'il est arrivé à destination.

    if d == m:
        return 1

    for i in positions:
        if m == i:
            return -1

    return 0


if __name__ == '__main__':
    d = int(input())
    m = int(input())
    n = int(input())
    positions = list(map(int, input().split()))
    print(state(d, m, n, positions))