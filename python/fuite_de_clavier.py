from typing import List

def fuite_de_clavier(n: int, k: int, chaine: List[str]) -> None:
    """
    :param n: taille de la chaîne
    :param k: taille du mot de passe
    :param chaine: la chaîne contenant le mot de passe
    """

    count = 0
    
    minuscules = 0
    majuscules = 0
    nombres = 0
    caracteres = 0

    for i in range(len(chaine)):
        if ord("a") <= ord(chaine[i]) <= ord("z"):
            minuscules += 1
        elif ord("A") <= ord(chaine[i]) <= ord("Z"):
            majuscules += 1
        elif ord("0") <= ord(chaine[i]) <= ord("9"):
            nombres += 1
        elif chaine[i] in "!\"#$%&'()*+,-./:;<=>?@[\]^_`{|}~":
            caracteres += 1

        if i >= k:
            if ord("a") <= ord(chaine[i-k]) <= ord("z"):
                minuscules -= 1
            elif ord("A") <= ord(chaine[i-k]) <= ord("Z"):
                majuscules -= 1
            elif ord("0") <= ord(chaine[i-k]) <= ord("9"):
                nombres -= 1
            elif chaine[i-k] in "!\"#$%&'()*+,-./:;<=>?@[\]^_`{|}~":
                caracteres -= 1

        if i >= k-1 and minuscules > 0 and majuscules > 0 and nombres > 0 and caracteres > 0:
            count += 1

    print(count)


if __name__ == "__main__":
    n = int(input())
    k = int(input())
    chaine = list(input())
    fuite_de_clavier(n, k, chaine)