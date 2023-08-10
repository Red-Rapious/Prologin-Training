from typing import List

def visite_sommet(s, vus, couplage, u):
    if vus[s]:
        return False
    else:
        vus[s] = True
        bv = existe(u[s], vus, couplage, u)
        if bv == None:
            return False
        else:
            couplage[bv] = s
            return True


def existe(liste, vus, couplage, u):
    for e in liste:
        if bon_voisin(e, vus, couplage, u):
            return e
    return None

def bon_voisin(v, vus, couplage, u):
    c = couplage[v]
    if c == None:
        return True
    else:
        return visite_sommet(c, vus, couplage, u)

def mise_en_boite(n: int, restes: List[int], boites: List[int]) -> None:
    """
    :param n: Le nombre de boîtes et de restes
    :param restes: Liste des volumes des restes
    :param boites: Liste des volumes des boîtes
    """
    # on traite ceci comme un problème de couplage maximal d'un graphe biparti
    restes = sorted(restes)
    boites = sorted(boites)[::-1]
    nu = len(restes)
    nv = len(boites)

    u = [[] for _ in range(nu)]
    
    for r in range(len(restes)):
        for b in range(len(boites)):
            if restes[r] <= boites[b]:
                u[r].append(b)
            else:
                break

    vus = [False] * nu
    couplage = [None] * nv

    for s in range(nu):
        for i in range(nu):
            vus[i] = False
        visite_sommet(s, vus, couplage, u)

    
    i = 0
    for e in couplage:
        if e != None:
            i += 1
    print(i)
    

if __name__ == "__main__":
    n = int(input())
    restes = list(map(int, input().split()))
    boites = list(map(int, input().split()))
    mise_en_boite(n, restes, boites)