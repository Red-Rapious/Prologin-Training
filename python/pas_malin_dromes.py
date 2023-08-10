from typing import List

def est_palindrome(mot):
    n = len(mot)//2

    for i in range(n):
        if mot[i] != mot[- i - 1]:
            return False

    return True

def nb_pas_malin_drome(n: int, mots: List[str]) -> None:
    """
    :param n: Le nombre de mots de passe contenus dans le fichier de mots de passe de Raphaël
    :param mots: La liste des mots de passe à décoder
    """
    
    mdp = 0

    for mot in mots:
        maju = ""
        minu = ""
        numb = ""

        for char in mot:
            if char.isdigit():
                numb += char
            elif char.isalpha():
                if char.upper() == char:
                    maju += char
                else:
                    minu += char

        if est_palindrome(maju) and est_palindrome(minu) and est_palindrome(numb):
            mdp += 1

    print(mdp)



if __name__ == "__main__":
    n = int(input())
    mots = [input() for _ in range(n)]
    nb_pas_malin_drome(n, mots)