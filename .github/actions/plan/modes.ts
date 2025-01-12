export type Mode = {
  name: string;
  cargoCommand: string;
  cargoArgs: string;
  cargoCacheKey: string;
  platformIndependent?: true;
};

export type Modes = Record<string, Mode>;

export const code = {
  clippy: {
    name: "clippy",
    cargoCommand: "clippy",
    cargoArgs: "--workspace --all-targets -- -D warnings",
    cargoCacheKey: "clippy",
  },
  test: {
    name: "test",
    cargoCommand: "test",
    cargoArgs: "--workspace",
    cargoCacheKey: "test",
  },
  build: {
    name: "build",
    cargoCommand: "build",
    cargoArgs: "--workspace",
    cargoCacheKey: "build",
  },
  fmt: {
    name: "fmt",
    cargoCommand: "fmt",
    cargoArgs: "-- --check",
    platformIndependent: true,
    cargoCacheKey: "code",
  },
  docs: {
    name: "doc",
    cargoCommand: "doc",
    cargoArgs: "--workspace --document-private-items",
    platformIndependent: true,
    cargoCacheKey: "doc",
  },
  testBenchmark: {
    name: "test benchmark",
    cargoCommand: "test",
    cargoArgs: "--workspace --features runtime-benchmarks",
    cargoCacheKey: "test-benchmark",
  },
  runBenchmark: {
    name: "test-run pallet benchmarks",
    cargoCommand: "run",
    cargoArgs:
      "-p humanode-peer --release --features runtime-benchmarks benchmark pallet --chain benchmark --execution native --pallet '*' --extrinsic '*' --steps 2 --repeat 0 --external-repeat 0",
    cargoCacheKey: "run-benchmark",
  },
  buildTryRuntime: {
    name: "build with try-runtime",
    cargoCommand: "build",
    cargoArgs: "--workspace --features try-runtime",
    cargoCacheKey: "try-runtime",
  },
} satisfies Modes;

export const build = {
  build: {
    name: "build",
    cargoCommand: "build",
    cargoArgs: "--workspace --release",
    cargoCacheKey: "release-build",
  },
} satisfies Modes;
