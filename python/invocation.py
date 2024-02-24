
from typing import List

def ith(syll_a, syll_b, i):
    if 0 <= i < len(syll_a):
        return syll_a[i]
    elif len(syll_a) <= i < len(syll_a) + len(syll_b):
        return syll_b[i - len(syll_a)]
    elif len(syll_a) + len(syll_b) <= i < 2 * len(syll_a) + len(syll_b):
        return syll_a[i - len(syll_a) - len(syll_b)]
    else:
        print("INVALID INDEX")
        exit(1)

def invocation(n: int, mots: List[str], m: int, vocabulaire: List[str]) -> None:
    """
    :param n: le nombre de débuts de sortilèges
    :param mots: la liste des débuts de sortilèges
    :param m: la taille du vocabulaire
    :param vocabulaire: le vocabulaire sous forme de liste de chaînes de caractères
    """
    sortileges = [0 for _ in range(n)]

    for syll_a in vocabulaire:
        for syll_b in vocabulaire:
            for mot in range(len(mots)):
                if len(mots[mot]) > 2*len(syll_a) + len(syll_b):
                    break
                correct = True
                for i in range(len(mots[mot])):
                    if mots[mot][i] != ith(syll_a, syll_b, i):
                        correct = False
                        break
                if correct:
                    sortileges[mot] += 1

    for s in sortileges:
        print(s)


if __name__ == "__main__":
    n = int(input())
    mots = [input() for _ in range(n)]
    m = int(input())
    vocabulaire = [input() for _ in range(m)]
    invocation(n, mots, m, vocabulaire)
