from typing import List


def nombre_films(adore: List[str], deteste: List[str]) -> None:
    """
    :param adore: liste des noms du film adoré de chaque personne
    :param deteste: liste des noms du film détesté de chaque personne
    """
    
    valable = []

    for film in adore:
        if film not in deteste and film not in valable:
            valable.append(film)
        
    print(len(valable))


if __name__ == "__main__":
    adore = [input() for _ in range(6)]
    deteste = [input() for _ in range(6)]
    nombre_films(adore, deteste)