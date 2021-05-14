// Import BenchmarkBase class.
import 'package:benchmark_harness/benchmark_harness.dart';
import 'package:memoize/memoize.dart';

int fibRecursive(int n) {
  if (n == 0) return 1;
  if (n == 1 || n == 2) return n;
  return fibRecursive(n - 1) + fibRecursive(n - 2);
}

int fibIterate(int n) {
  var pseudoCache = List.filled(n, 0);
  pseudoCache[0] = 1;
  pseudoCache[1] = 2;
  for (var i = 2; i < n; i++) {
    pseudoCache[i] = pseudoCache[i - 1] + pseudoCache[i - 2];
  }
  return pseudoCache.last;
}

class RecursiveNoCacheBenchmark extends BenchmarkBase {
  const RecursiveNoCacheBenchmark() : super('RecursiveNoCache');

  static void main() {
    RecursiveNoCacheBenchmark().report();
  }

  @override
  void run() {
    fibRecursive(35);
  }
}

class RecursiveCachedBenchmark extends BenchmarkBase {
  RecursiveCachedBenchmark() : super('RecursiveCached');
  Function fibCached;

  static void main() {
    RecursiveCachedBenchmark().report();
  }

  @override
  void run() {
    fibCached(35);
  }

  @override
  void setup() {
    fibCached = memo1(fibRecursive);
  }
}

class IterateBenchmark extends BenchmarkBase {
  const IterateBenchmark() : super('Iterate');

  static void main() {
    IterateBenchmark().report();
  }

  @override
  void run() {
    fibIterate(35);
  }
}

void main() {
  RecursiveNoCacheBenchmark.main();
  RecursiveCachedBenchmark.main();
  IterateBenchmark.main();
}
