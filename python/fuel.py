nb_personnes = int(input())

def fuel(n, p):
    """
    :param n: le nombre de membres dans l'équipage
    :type n: int
    :param p: le poids de chaque passager
    :type p: list[int]
    """
    
    carb = 0
    for i in p:
        if i>90:
            carb += 80
        else:
            carb += 60
        
    
    return carb


if __name__ == '__main__':
    #n = int(input())
    n=0
    p = list(map(int, input().split()))
    print(fuel(n, p))