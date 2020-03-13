class UnionFind:
	def __init__(self, N):
		self.setSize = [1] * N
		self.numSets = N
		self.rank = [0] * N
		self.p = list(range(N))
	
	def findSet(self, i):
		if self.p[i] == i:
			return i
		else:
			self.p[i] = self.findSet(self.p[i])
			return self.p[i]

	def isSameSet(self, i, j):
		return self.findSet(i) == self.findSet(j)

	def unionSet(self,i, j):
		if not self.isSameSet(i, j):
			self.numSets-=1
		x = self.findSet(i)
		y = self.findSet(j)
		if self.rank[x] > self.rank[y]:
			self.p[y] = x
			self.setSize[x] += self.setSize[y]
		else:
			self.p[x] = y
			self.setSize[y] += self.setSize[x]
			if self.rank[x] == self.rank[y]:
				self.rank[y]+=1
	def numDisjointSets(self):
		return self.numSets
	
	def sizeOfSet(self, i):
		return self.setSize[self.findSet(i)]


if __name__ == "__main__":
	print('Assume that there are 5 disjoint sets initially')
	UF = UnionFind(5) 
	print("{}".format(UF.numDisjointSets()))
	UF.unionSet(0, 1)
	print("{}".format(UF.numDisjointSets()))
	UF.unionSet(2, 3)
	print("{}".format(UF.numDisjointSets()))
	UF.unionSet(4, 3)
	print("{}".format(UF.numDisjointSets()))
	print("isSameSet(0, 3) = {}".format(UF.isSameSet(0, 3))) # will return 0 (false)
	print("isSameSet(4, 3) = {}".format(UF.isSameSet(4, 3))) # will return 1 (true)
	for i in range(5): # findSet will return 1 for {0, 1} and 3 for {2, 3, 4}
		print("findSet({}) = {}, sizeOfSet({}) = {}".format(i, UF.findSet(i), i, UF.sizeOfSet(i)))
	UF.unionSet(0, 3)
	print("{}".format(UF.numDisjointSets())) # 1
	for i in range(5): # findSet will return 3 for {0, 1, 2, 3, 4}
		print("findSet({}) = {}, sizeOfSet({}) = {}".format(i, UF.findSet(i), i, UF.sizeOfSet(i)))
