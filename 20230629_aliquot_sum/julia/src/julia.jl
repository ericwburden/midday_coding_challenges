module julia

# Caching -----------------------------------------------------------------------------
# Here we set up to cache results from previous calculations to disk. Normally, if this
# is something you want to do in "real-life", you'd use something like a Redis instance
# or other key-value data store for this. Here, to keep things simple, I'm serializing
# a Dict to disk as our key-value store.

using Serialization

const CACHE_FILE = "cache.dict"

@enum Classification deficient perfect abundant

const Cache = Dict{UInt,Classification}

function read_cache()::Cache
  isfile(CACHE_FILE) && return deserialize(CACHE_FILE)
  Cache()
end

function write_cache(cache::Cache)
  serialize(CACHE_FILE, cache)
end

# Number Classification ---------------------------------------------------------------

# Compute the aliquot sum of the given number. The aliquot sum is the sum of all the
# factors of that number, including '1', but not including the number itself.
function aliquot_sum(n::UInt)::UInt
  limit = n |> sqrt |> floor |> Int
  total = 1

  for number = 2:limit 
    if n % number == 0
      total += number
      total += Int(n / number)
    end
  end

  total
end


# Various methods on the `classify` function that take a variety of different arguments
# and return either a single or multiple classifications, depending on the argument.
function classify(n::Int)::Classification
  n > 0 || error("Classification is only meaningful for numbers greater than zero.")
  classify(UInt(n))
end

function classify(ns::Vector{Int})::Vector{Classification}
  [classify(UInt(n)) for n in ns]
end

function classify(n::UInt)::Classification
  # Here, we read and write from the disk to classify a single number. 
  cache  = read_cache()
  result = classify!(n, cache)
  write_cache(cache)
  result
end

function classify(ns::Vector{UInt})::Vector{Classification}
  # Here, we still read and write from disk only a single time for multiple numbers.
  cache   = read_cache()
  results = [classify!(n, cache) for n in ns]
  write_cache(cache)
  results
end

# This is the main function. If the number requested is in the cache, just return
# the cached classification. If not, calculate the classification by comparing the
# number to its aliquot sum and making the correct classification. The cache is 
# updated if a new calculation is performed.
function classify!(n::UInt, cache::Dict{UInt,Classification})::Classification
  haskey(cache, n) && return cache[n]

  aliquot = aliquot_sum(n)
  result  = perfect
  n > aliquot && (result = deficient)
  n < aliquot && (result = abundant)

  cache[n] = result
  result
end

end # module julia
