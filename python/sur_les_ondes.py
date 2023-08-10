from typing import List


def sur_les_ondes(n: int, freqs: List[int]) -> None:
    """
    :param n: nombre de fréquences données
    :param freqs: la liste des fréquences à vérifier
    """
    freqs = sorted(freqs)
    for f in freqs:
        if f % 3 == 0:
            print(f)
            break


if __name__ == "__main__":
    n = int(input())
    freqs = list(map(int, input().split()))
    sur_les_ondes(n, freqs)
