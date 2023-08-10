def expert_itinerant(nb_sommets, nb_arretes, nb_requetes, D, A, T, d, a):
    """ Complétez le code ici. Vous devez print() R lignes. """

    for request in range(nb_requetes):
        # sommets[0] est arbitraire et uniquement là pour faciliter l'utilisation des indices
        sommets = [None] + [float("inf") for i in range(nb_sommets)]
        sommets_parcourus = [] # complémentaire de l'ensemble de sommets Q
        sommets[d[request]] = 0
        predecesseur = [None] + [None for i in range(nb_sommets)]

        while len(sommets) != len(sommets_parcourus):
            # recherche du minimum
            sommet_le_plus_proche = (float("inf"), None)
            for som in range(1, len(sommets)):
                if som not in sommets_parcourus and sommets[som] < sommet_le_plus_proche[0]:
                    sommet_le_plus_proche = (sommets[som], som)
            sommets_parcourus.append(sommet_le_plus_proche[1])

            # mise à jour des distances
            for depart in range(len(D)):
                if D[depart] == sommet_le_plus_proche[1]:
                    sommet_adjacent = A[depart]
                    if sommets[sommet_adjacent] > sommets[sommet_le_plus_proche[1]] + T[depart]:
                        sommets[sommet_adjacent] = sommets[sommet_le_plus_proche[1]] + T[depart]
                        predecesseur[sommet_adjacent] = sommet_le_plus_proche[1]
        
        print(sommets[a[request]])
        
        

if __name__ == '__main__':
    N, M, R = (int(i) for i in input().split())

    D = []
    A = []
    T = []
    for _ in range(M):
        my_D, my_A, my_T = (int(i) for i in input().split())
        D.append(my_D)
        A.append(my_A)
        T.append(my_T)

    d = []
    a = []
    for _ in range(R):
        my_d, my_a = (int(i) for i in input().split())
        d.append(my_d)
        a.append(my_a)

    expert_itinerant(N, M, R, D, A, T, d, a)