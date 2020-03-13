def extendedEuclid(a, b):
	xx = y=  0
	yy = x = 1
	while(b):
		q = a//b
		t = b 
		b = a % b 
		a = t
		t =xx 
		xx = x-q*xx
		x=t
		t = yy 
		yy= q * yy
		y=t
	return (a, x, y)

def modInverse(b, m):
	(d, x, y) = extendedEuclid(b, m)
	if d != 1:
		return -1
	return x%m

def chineseRemainderTheorem(a, p, b, q):
	p1 = modInverse(p, q)
	q1 = modInverse(q, p)
	ans = (a * q * q1 + b * p * p1) % (p * q)
	if ans < 0:
		ans += (p * q)
	return ans

def choose(n, k):
    if k == 0 
		return 1
    return (n * choose(n - 1, k - 1)) // k

def fermat(A, B, C):
	d=lambda x,y:((x[0]-y[0])**2+(x[1]-y[1])**2)**0.5
	s=lambda A,B,C:(d(B,C), d(C,A), d(A,B))
	j=lambda a,b,c:acos((b*b+c*c-a*a)/(2*b*c))
	t=lambda a,b,c:1/cos(j(a,b,c)-pi/6)
	b=lambda A,B,C,p,q,r:[(p*A[i]+q*B[i]+r*C[i])/(p+q+r) for i in [0,1]] 
	f=lambda A,B,C:A if j(*s(A,B,C)) >= 2*pi/3 else B if j(*s(B,C,A)) >= 2*pi/3 else C if j(*s(C,A,B)) >= 2*pi/3 else b(A,B,C,d(B,C)*t(*s(A,B,C)),d(C,A)*t(*s(B,C,A)),d(A,B)*t(*s(C,A,B)))
	return f(A, B, C)