n = int(input())
matrice = []

path_matrice = [[None for i in range(n)] for j in range(n)]

for _ in range(n):
    matrice.append([int(s) for s in input().split()])

def calculate_max_path(x,y):
    if x==0 and y==0:
        return matrice[x][y]
    elif x==0:
        return matrice[x][y] + max_path(x,y-1)
    elif y==0:
        return matrice[x][y] + max_path(x-1,y)
    else:
        return matrice[x][y] + max(max_path(x-1,y), max_path(x, y-1))


def max_path(x,y):
    if path_matrice[x][y] == None:
        path = calculate_max_path(x,y)
        path_matrice[x][y] = path
        return path
    else:
        return path_matrice[x][y]

print(max_path(n-1,n-1))