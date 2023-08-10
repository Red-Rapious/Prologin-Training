def marche_nocturne(n, liste_prix, b):
    """
    :param n: nombre de minerai rare
    :type n: int

    :param liste_prix: liste des prix de chaque minerai
    :type liste_prix: list[int]

    :param b: le total d'argent que vous souhaitez dépenser dans ce souvenir
    :type b: int
    """
    nb_min = -1
    b_actuel = 0
    j=0

    for i in range(len(liste_prix)):
        j=0
        b_actuel = 0
        
        while b_actuel < b:
            b_actuel += liste_prix[j+i]
            j+=1

            if j+i>=len(liste_prix):
                break
        if b_actuel == b and (j<nb_min or nb_min == -1):
            nb_min=j
        if nb_min == 1:
            return 1

    return nb_min


if __name__ == '__main__':
    n = int(input())
    liste_prix = list(map(int, input().split()))
    b = int(input())
    print(marche_nocturne(n, liste_prix, b))