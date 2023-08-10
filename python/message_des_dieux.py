def trouver_lettre(t, s):
    """
    :param t: taille de la séquence
    :type t: int
    :param s: liste des entiers que Hermès a sorti
    :type s: list[int]
    """
    if s[0]<65 or s[0]>(65+26):
        return " "
    l=s[0]-65
    somme=0
    for i in s[1:]:
        somme+=i
    somme=somme%26
    l=(l+somme)%30+65
    
    return chr(l)


if __name__ == '__main__':
    t = int(input())
    s = list(map(int, input().split()))
    print(trouver_lettre(t, s))