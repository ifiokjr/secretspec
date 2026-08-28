namespace Monosecret;

/// <summary>Entry point for the Monosecret C# SDK.</summary>
public static class Monosecret
{
    /// <summary>Starts a fluent resolution builder.</summary>
    public static MonosecretBuilder Builder() => new();

    /// <summary>Resolves secrets in one call.</summary>
    public static Resolved Resolve(
        string? path = null,
        string? provider = null,
        string? profile = null,
        string? reason = null,
        string? scope = null,
        CallerContext? caller = null) =>
        Configured(path, provider, profile, scope, reason, caller).Load();

    /// <summary>Builds a value-free inventory report in one call.</summary>
    public static ResolutionReport Report(
        string? path = null,
        string? provider = null,
        string? profile = null,
        string? reason = null,
        string? scope = null,
        CallerContext? caller = null) =>
        Configured(path, provider, profile, scope, reason, caller).Report();

    /// <summary>The ABI version reported by the loaded native resolver.</summary>
    public static string AbiVersion() => Native.AbiVersion();

    private static MonosecretBuilder Configured(
        string? path,
        string? provider,
        string? profile,
        string? scope,
        string? reason,
        CallerContext? caller) =>
        Builder()
            .WithPath(path)
            .WithProvider(provider)
            .WithProfile(profile)
            .WithScope(scope)
            .WithReason(reason)
            .WithCaller(caller);
}
