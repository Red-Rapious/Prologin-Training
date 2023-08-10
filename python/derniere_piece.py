def resoudre(ncouleurs, couleurs, ncotes, couleurscotes, npieces, pieces):
    """
    :param ncouleurs: le nombre de couleurs
    :type ncouleurs: int
    :param couleurs: les différentes couleurs possibles
    :type couleurs: list[str]
    :param ncotes: le nombre de côtés de la pièce manquante
    :type ncotes: int
    :param couleurscotes: les couleurs des pièces adjacentes à la pièce manquante
    :type couleurscotes: list[str]
    :param npieces: le nombre de pièces à trier
    :type npieces: int
    :param pieces: les pièces à trier
    :type pieces: list[dict["nCotesPiece": int, "couleurPiece": str]]
    """
    s=""
    c=0
    for p in pieces:
        if p["nCotesPiece"] == ncotes and p["couleurPiece"] not in couleurscotes:
            s+="O"
            c+=1
        else:
            s+="X"
    print(s)
    print(c)


if __name__ == '__main__':
    ncouleurs = int(input())
    couleurs = [input() for _ in range(ncouleurs)]
    ncotes = int(input())
    couleurscotes = [input() for _ in range(ncotes)]
    npieces = int(input())
    pieces = [{
        "nCotesPiece": int(input()),
        "couleurPiece": input()
    } for _ in range(npieces)]
    resoudre(ncouleurs, couleurs, ncotes, couleurscotes, npieces, pieces)