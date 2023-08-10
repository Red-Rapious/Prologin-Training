def boite_aux_lettres(nom, n, noms):
    """
    :param nom: nom erroné noté sur le colis
    :type nom: str
    :param n: nombre de boîtes aux lettres nom erroné noté sur le colis
    :type n: int
    :param noms: N noms de familles
    :type noms: list[str]
    """

    for boite in noms:
        if len(boite) == len(nom):
            erreurs = 0
            for i in range(len(nom)):
                if not boite[i] == nom[i]:
                    erreurs +=1
            if erreurs <= 3:
                 return boite


if __name__ == '__main__':
    nom = input()
    n = int(input())
    noms = [input() for _ in range(n)]
    print(boite_aux_lettres(nom, n, noms))