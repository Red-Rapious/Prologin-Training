def capitalisme_interplanetaire(t, n, planetes, d, mission):
    """
    :param t: nombre de types de planètes distincts
    :type t: int

    :param n: le nombre de planètes dans le système
    :type n: int

    :param planetes: les planètes du système
    :type planetes: list[dict["x": int, "y": int, "k": int]]

    :param d: la durée de la mission à traiter
    :type d: int

    :param mission: pour chaque jour de la mission, le type de planète à visiter
    :type mission: list[int]
    """

    p = get_p_from_type(planetes, mission[-1])
    table = [(p[i], 0) for i in range(len(p))]

    for i in range(1, d):
        
        p = get_p_from_type(planetes, mission[-i-1])
        table = [(p[j], minimum(p[j], table)) for j in range(len(p))]

    min_dist = -1
    for i in table:
        if min_dist==-1 or i[1]<min_dist:
            min_dist=i[1]

    return min_dist


def get_p_from_type(planetes, ptype): # return a list of each planete wich type is the given ptype
    new_plan = []
    for i in range(len(planetes)):
        if planetes[i]['k'] == ptype:
            new_plan.append(planetes[i])
    return new_plan



def minimum(iplanete, table):
    mini = -1

    for i in table:
        d= man(i[0]['x'], i[0]['y'], iplanete['x'], iplanete['y'])
        if d+i[1]<mini or mini==-1:
            mini=d+i[1]

    return mini



def abs(x):
    if x<0:
        return -x
    else:
        return x

def man(xa, ya, xb, yb):
    return abs(xa-xb)+abs(ya-yb)

if __name__ == '__main__':
    t = int(input())
    n = int(input())
    planetes = [dict(zip(("x", "y", "k"), map(int, input().split()))) for _ in range(n)]
    d = int(input())
    mission = list(map(int, input().split()))
    print(capitalisme_interplanetaire(t, n, planetes, d, mission))