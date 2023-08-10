visited = []

isolated = []
gh = 0
gl = 0
card = []

def cafe_asteroide(h, l, p, carte):
    """
    :param h: hauteur de la carte (longueur)
    :type h: int
    :param l: largeur de la carte
    :type l: int
    :param p: position du vaisseau sur la première ligne de la carte
    :type p: int
    :param carte: tableau représentant la carte des astéroïdes, des '*' pour les asteroides, des '.' pour du vide
    :type carte: list[list[str]]
    """
    global visited
    global isolated
    global gh
    global gl
    global card

    visited = [[None for i in range(h)] for j in range(l)]
    isolated = [[False for i in range(h)] for j in range(l)]
    gh = h
    gl = l


    carte2 = [[False for i in range(h)] for j in range(l)]

    for i in range(h):
        for j in range(l):
            if carte[i][j] == '.':
                carte2[j][i] = True
            else:
                carte2[j][i] = False

    card = carte2.copy()


    if can_go(p,0):
        print('May the force be with you')
    else:
        print('I have a bad feeling about this')

def can_go(x, y):
    if isolated[x][y]:
        return False

    elif visited[x][y] != None:
        isolated[x][y] = True
        return visited[x][y]

    else:
        result = None
        isolated[x][y] = True
        if not card[x][y]:
            result = False

        elif y==gh-1:
            result = True

        elif x == 0:
            result = can_go(x, y+1) or can_go(x+1,y+1)
        elif x== gl-1:
            result = can_go(x, y+1) or can_go(x-1,y+1)

        else:
            result = can_go(x, y+1) or can_go(x-1,y+1) or can_go(x+1, y+1)

        visited[x][y] = result

        return result


if __name__ == '__main__':
    h = int(input())
    l = int(input())
    p = int(input())
    cartes = [list(input()) for _ in range(h)]
    cafe_asteroide(h, l, p, cartes)