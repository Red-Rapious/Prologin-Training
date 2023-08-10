def champ_asteroides(v, c, n, tailles):
    """
    :param v: Volume de carburant initial
    :type v: int
    :param c: Quantité nécessaire pour éviter un astéroïde de taille 1.
    :type c: int
    :param n: Nombre d'astéroïdes sur le chemin
    :type n: int
    :param tailles: Liste des tailles d'astéroïdes
    :type tailles: list[int]
    """

    total = 0

    for i in tailles:
        max = i*c
        if i <= 20 and max>40:
            max = 40
        elif i >= 80 and max >= 120:
            max = 120

        total += max

    if total < v:
        return total
    else:
        return -1

if __name__ == '__main__':
    v = int(input())
    c = int(input())
    n = int(input())
    tailles = list(map(int, input().split()))
    print(champ_asteroides(v, c, n, tailles))