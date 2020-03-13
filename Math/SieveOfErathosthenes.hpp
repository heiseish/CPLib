#include <bitset>
#include <vector>

class Sieve {
private:
	long long _sieve_size;
	std::bitset<10000010> bs;                     
	std::vector<long long> primes;  
	std::vector<long long> eulerPhi;          
public:
	Sieve (long long upperbound) : _sieve_size(upperbound + 1)  {     
		bs.set();                                                
		bs[0] = bs[1] = 0;                                   
		for (long long i = 2; i < _sieve_size; ++i) 
			if (bs[i]) {
				for (long long j = i*i; j < _sieve_size; j += i) 
				bs[j] = 0;
				primes.push_back(i);       
			} 
	}                                    

	bool isPrime(long long N) {                 
		if (N < _sieve_size) return bs[N];              
		for (int i = 0; (i < primes.size()) && (primes[i]*primes[i] <= N); i++)
			if (N%primes[i] == 0) return false;
		return true;                 
	}          

	vector<int> primeFactors(long long N) {  
		vector<int> factors;                 
		long long PF_idx = 0, PF = primes[PF_idx];     
		while ((N != 1) && (PF*PF <= N)) {   
			while (N%PF == 0) { 
				N /= PF; factors.push_back(PF); 
			}   
			PF = primes[++PF_idx];              
		}
		if (N != 1 && ) factors.push_back(N);
		return factors;  
	}

	long long numPF(long long N) {
		long long PF_idx = 0, PF = primes[PF_idx], ans = 0;
		while (N != 1 && (PF*PF <= N)) {
    		while (N%PF == 0) { N /= PF; ans++; }
    			PF = primes[++PF_idx];
  		}
  		return ans + (N != 1);
	}

	long long numDiffPF(long long N) {
  		long long PF_idx = 0, PF = primes[PF_idx], ans = 0;
  		while (N != 1 && (PF*PF <= N)) {
    		if (N%PF == 0) ans++;                
    		while (N%PF == 0) N /= PF;
    		PF = primes[++PF_idx];
  		}
  		return ans + (N != 1);
	}

	long long sumPF(long long N) {
		long long PF_idx = 0, PF = primes[PF_idx], ans = 0;
  		while (N != 1 && (PF*PF <= N)) {
    		while (N%PF == 0) { N /= PF; ans += PF; }
    		PF = primes[++PF_idx];
  		}
  		return ans + (N != 1) * N;
	}

	long long numDiv(long long N) {
  		long long PF_idx = 0, PF = primes[PF_idx], ans = 1;             // start from ans = 1
  		while (N != 1 && (PF*PF <= N)) {
			long long power = 0;                                             // count the power
			while (N%PF == 0) { 
				N /= PF; 
				power++; 
			}
			ans *= (power+1);                                // according to the formula
			PF = primes[++PF_idx];
		}
	return (N != 1) ? 2*ans : ans;    // (last factor has pow = 1, we add 1 to it)
	}

	long long sumDiv(long long N) {
		long long PF_idx = 0, PF = primes[PF_idx], ans = 1;             // start from ans = 1
		while (N != 1 && (PF*PF <= N)) {
			long long power = 0;
			while (N%PF == 0) { N /= PF; power++; }
			ans *= ((long long)pow((double)PF, power+1.0) - 1) / (PF-1);             // formula
			PF = primes[++PF_idx];
		}
		if (N != 1) ans *= ((long long)pow((double)N, 2.0) - 1) / (N-1);          // last one
		return ans;
	}



	void precomputeEulerPhi(int _max_eulerphi) {
		eulerPhi.assign(_max_eulerphi, 0)
		for(int i = 1; i < _max_eulerphi; ++i) eulerPhi[i] = i;
		for(int i = 2; i < _max_eulerphi; ++i) 
			if (eulerPhi[i] == i)
				for(int j = i; j < _max_eulerphi; j += i)
					eulerPhi[j] = (eulerPhi[j]/i) * (i - 1);
	}

	long long getPrecomputedEulerPhi(int n) {
		return eulerPhi[n];
	}

	long long EulerPhi(long long N) {
		long long PF_idx = 0, PF = primes[PF_idx], ans = N;             // start from ans = N
		while (N != 1 && (PF * PF <= N)) {
			if (N % PF == 0) ans -= ans / PF;                // only count unique factor
			while (N % PF == 0) N /= PF;
			PF = primes[++PF_idx];
		}
		return (N != 1) ? ans - ans/N : ans;                            // last factor
	}
} 
