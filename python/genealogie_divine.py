def genealogie_divine(f, m, familles, c, cartes):
    """
    :param f: Le nombre de familles
    :type f: int
    :param m: Le nombre de membres par famille
    :type m: int
    :param familles: Les différentes familles possibles dans le jeu
    :type familles: list[list[str]]
    :param c: Le nombre de cartes dans la main de Joseph
    :type c: int
    :param cartes: Les cartes dans la main de Joseph
    :type cartes: list[str]
    """
    mini = -1
    for fam in familles:
        manquant = 0
        testes = []
        for c in fam:
            if not c in testes and not c in cartes :
                testes.append(c)
                manquant += 1

        if mini == -1 or manquant < mini:
            mini = manquant
    
    return mini


if __name__ == '__main__':
    f = int(input())
    m = int(input())
    familles = [list(input()) for _ in range(f)]
    c = int(input())
    cartes = list(input())
    print(genealogie_divine(f, m, familles, c, cartes))