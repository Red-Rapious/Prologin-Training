def interferences(n, s):
    """
    :param n: la longueur du message
    :type n: int
    :param s: le message
    :type s: str
    """
    s=s.replace(".", "")
    new_s = []
    reading=True
    
    for i in s:
        if i == '*':
            if reading:
                reading=False
            else:
                reading=True
        elif reading:
            new_s.append(i)
    
    for i in new_s:
	    print(i, end='')


if __name__ == '__main__':
    n = int(input())
    s = input()
    interferences(n, s)