from typing import List


def trajets_retour(n: int, redirection: List[int]) -> None:
    """
    :param n: le nombre de cinémas
    :param redirection: le lieu de redirection de chaque cinéma
    """
    
    for dep in range(1, n+1):
        visite = [False] * n
        tot = 0
        cine = dep

        while not visite[cine - 1]:
            tot += 1
            visite[cine - 1] = True
            cine = redirection[cine - 1]

        print(tot, end=(" " if dep != n else ""))


if __name__ == "__main__":
    n = int(input())
    redirection = list(map(int, input().split()))
    trajets_retour(n, redirection)