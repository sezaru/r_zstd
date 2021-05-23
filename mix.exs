defmodule RZstd.MixProject do
  use Mix.Project

  @app :r_zstd
  @name "RZstd"
  @description "Rust powered NIF bindings to Facebook's Zstd library"
  @version "1.1.0-rc.4"
  @github "https://github.com/sezaru/#{@app}"
  @author "Eduardo Barreto Alexandre"
  @license "MIT"

  def project do
    [
      app: @app,
      version: @version,
      elixir: "~> 1.10",
      name: @name,
      description: @description,
      deps: deps(),
      docs: docs(),
      package: package(),
      # TODO Revert this after Rustler resolves so missing bug (issue 326)
      build_embedded: false,
      start_permanent: Mix.env() == :prod,
      compilers: Mix.compilers(),
      rustler_crates: rustler_crates(),
      elixirc_paths: elixirc_paths(Mix.env()),
      test_coverage: [tool: ExCoveralls],
      preferred_cli_env: preferred_cli_env(),
      dialyzer: dialyzer()
    ]
  end

  def application, do: [extra_applications: [:logger]]

  defp deps do
    [
      {:benchee, "~> 1.0", only: :dev},
      {:unsafe, "~> 1.0", runtime: false},
      {:rustler, "~> 0.22.0-rc.1"},
      {:ex_doc, "~> 0.24.2", only: [:dev, :docs], runtime: false},
      {:excoveralls, "~> 0.13", only: [:dev, :test], runtime: false},
      {:dialyxir, "~> 1.0", only: [:dev, :test], runtime: false},
      {:credo, "~> 1.4", only: [:dev, :test], runtime: false}
    ]
  end

  defp docs do
    [
      main: "readme",
      source_ref: "v#{@version}",
      source_url: @github,
      extras: [
        "README.md"
      ]
    ]
  end

  defp package do
    [
      name: @app,
      maintainers: [@author],
      licenses: [@license],
      files: ~w(mix.exs lib README.md native/Cargo.* native/src),
      links: %{"Github" => @github}
    ]
  end

  defp rustler_crates do
    [
      r_zstd_nif: [
        path: "native",
        cargo: :system,
        default_features: false,
        features: [],
        mode: :release
      ]
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp preferred_cli_env do
    [
      coveralls: :test,
      "coveralls.detail": :test,
      "coveralls.post": :test,
      "coveralls.html": :test
    ]
  end

  defp dialyzer, do: [plt_add_apps: [:mix, :eex]]
end
