import random

# Define parameters
p = 3
k = 3
d = 4
n = 2**d - 1  # Number of participants
t = 5 # Threshold
assert(t>=d)


# Construct the base ring Z/2^kZ
R = IntegerModRing(2**k)

# Find an irreducible polynomial of degree d over GF(2)
F = GF(2)
P = PolynomialRing(F, 'x')
x = P.gen()
h = P.irreducible_element(d)

# Construct the Galois ring GR(2^k, d) as R[x]/(h(x))
GR = PolynomialRing(R, 'X')
X = GR.gen()
GaloisRing = GR.quotient(h, 'X')

# Generate the field GF(2^d)

def generate_exceptional_set(d):
    GF_2d = GF(2^d, name='X')
    exceptional_set = []
    for elem in GF_2d:
        if elem != 0:
            exceptional_set.append(elem)

    print("Exceptional Set:")
    for i, elem in enumerate(exceptional_set):
        print(f"Point {i}: {elem}")
    return exceptional_set

# Function to create a random polynomial of degree t-1
def random_polynomial(t, secret, ring):
    print("RING_random_poly", ring)
    coeffs = [secret] + [ring.random_element() for _ in range(t - 1)]
    print("Random polynomial Coeefs", coeffs)
    return coeffs

# Function to manually evaluate polynomial at a given point
def evaluate_polynomial(poly, point):
    term = GaloisRing(0)
    for i, coeff in enumerate(poly):
        term += coeff * (GaloisRing(point) ^ i)
    print(f"Evaluating polynomial {poly} at point {point} gives {term}")  # Debugging: print evaluation result
    return term



# Function to convert a Galois ring element back to an integer
def galois_ring_element_to_integer(poly):
    result = 0
    for i, coeff in enumerate(poly.list()):
        result += int(coeff) * (2**i)
    return result

   
# Shamir's Secret Sharing
def shamir_share(secret, t, n, exceptional_set, ring):
    print("Ring Shamir Share", ring)
    poly = random_polynomial(t, secret, GaloisRing)
    #poly = [secret , 4*X^2  + 2, 3*X]
    shares = []
    for i in range(n):
        point = exceptional_set[i]
        share_value =(point, evaluate_polynomial(poly, point))
        shares.append(share_value)
    return shares




def find_inverse_in_galois_ring(elem, modulus):
    # Compute the GCD and the coefficients for the inverse using the extended Euclidean algorithm
    g, u, _ = xgcd(elem.change_ring(ZZ), modulus.change_ring(ZZ))
  
    print(f"before modulus: elem:{elem}, inverse: {u}, modulus:{modulus}")

    # The inverse of elem modulo modulus is given by u
    inverse = GaloisRing(u)/g
    inverse = GaloisRing(inverse)
    print(f"elem:{elem}, inverse: {u% modulus}, modulus:{modulus}")
    return inverse

# Lagrange Interpolation for reconstruction with unit check
def lagrange_interpolation(shares, ring):
    print("Shamir shares: ",shares)
    x = ring.gen()
    terms = []
    for  (xi, yi) in shares:
        li = ring(1)
        for (xj, _) in shares:
            if xi != xj:
                numerator =   ring(xj)
                print("xi=",xi, "xj=",xj)
                print("numerator", numerator)
         #      denominator =  ring(xj - xi)
                denominator =  ring(xj).lift() - ring(xi).lift()
                denominator_inv = find_inverse_in_galois_ring(denominator,GR(h))
                print(f"denominator:{denominator}, denominator_inv:{denominator_inv}")
                frac = numerator * ring(denominator_inv)
                print("frac:",frac)
                li *= frac
                print("li:",li)
                print("")
        terms.append(yi * li)
        print(f" yi: {yi}, li: {li}")
    result = sum(terms)
    reconstructed_secret = result
    print("Reconstructed poly :", result)
    return reconstructed_secret



# Secret to share

ring = GaloisRing
secret = ring.random_element()

exceptional_set = generate_exceptional_set(d)

# Generate shares
shares = shamir_share(secret, t, n, exceptional_set, GaloisRing)
print("Shares:")
vector_shares = []
for i, share in shares:
    vector_shares.append(share)
    print(f"Share {i}: {share}")

# Randomly select t shares for reconstruction
selected_shares = random.sample(shares, t)

# Print selected shares for debugging
print("Selected Shares for Reconstruction:")
for i, share in selected_shares:
    print(f"Share {i}")

# Reconstruct the secret
reconstructed_secert = lagrange_interpolation(selected_shares, GaloisRing)
print(f"Original Secret: {secret}")




