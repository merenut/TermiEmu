# Security Policy

## Supported Versions

TermiEmu is currently in active development (Pre-Alpha/Alpha stage). We provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| main    | :white_check_mark: |
| < 1.0   | :x:                |

Once version 1.0 is released, this policy will be updated to reflect long-term support for stable versions.

## Reporting a Vulnerability

We take the security of TermiEmu seriously. If you discover a security vulnerability, please follow these steps:

### 1. **Do Not** Open a Public Issue

Please do not report security vulnerabilities through public GitHub issues, discussions, or pull requests.

### 2. Report via GitHub Security Advisories (Preferred)

The preferred method for reporting vulnerabilities is through GitHub's private security advisory feature:

1. Navigate to the [Security Advisories](https://github.com/merenut/TermiEmu/security/advisories) page
2. Click "Report a vulnerability"
3. Fill out the form with details about the vulnerability
4. Submit the report

### 3. Alternative: Email Report

If you prefer not to use GitHub's advisory system, you can email security concerns to:

**Email:** [To be configured - maintainer email address]

Please encrypt sensitive information using our PGP key (if available) or use GitHub's private advisory feature.

### What to Include in Your Report

To help us understand and address the issue quickly, please include:

- **Type of vulnerability** (e.g., buffer overflow, injection, authentication bypass)
- **Full paths of source file(s)** related to the vulnerability
- **Location of the affected source code** (tag/branch/commit or direct URL)
- **Step-by-step instructions to reproduce** the issue
- **Proof-of-concept or exploit code** (if available)
- **Impact of the vulnerability** - what can an attacker do?
- **Any suggested fixes** (if you have them)

### Response Timeline

We aim to respond to security reports according to the following timeline:

- **Initial Response:** Within 48 hours
- **Assessment:** Within 7 days (determining severity and impact)
- **Fix Development:** Within 30 days (for critical vulnerabilities)
- **Public Disclosure:** After fix is available (coordinated disclosure)

### What to Expect

1. **Acknowledgment:** We'll acknowledge receipt of your vulnerability report
2. **Assessment:** We'll assess the vulnerability and determine its severity
3. **Communication:** We'll keep you informed of our progress
4. **Credit:** With your permission, we'll credit you in release notes and security advisories
5. **Disclosure:** We practice coordinated disclosure and will work with you on timing

## Security Best Practices for Users

While we work hard to keep TermiEmu secure, users should also follow security best practices:

### Running Untrusted Commands

- Be cautious when pasting commands from untrusted sources
- Review commands before executing them
- Use bracketed paste mode to prevent automatic command execution

### Configuration Files

- Protect your TermiEmu configuration files from unauthorized access
- Be careful when using configuration files from untrusted sources
- Review theme files and plugins before installation

### Updates

- Keep TermiEmu updated to the latest version
- Enable Dependabot alerts if you're building from source
- Review security advisories in our [GitHub Security tab](https://github.com/merenut/TermiEmu/security)

## Security Features

TermiEmu implements several security features:

### Current Implementation

- **Dependency Scanning:** Automated vulnerability scanning with cargo-audit
- **Supply Chain Security:** cargo-deny for license and dependency verification
- **Automated Updates:** Dependabot for keeping dependencies current
- **Memory Safety:** Rust's ownership system prevents many common vulnerabilities

### Planned Features (Roadmap)

- **Sandboxing:** Process isolation for enhanced security
- **Input Validation:** Comprehensive validation of escape sequences
- **Audit Logging:** Optional security event logging
- **Permission System:** Fine-grained permission controls

## Known Security Considerations

### PTY Security

Terminal emulators have inherent security considerations due to their interaction with PTYs and child processes:

- **Escape Sequence Injection:** Malicious escape sequences can affect terminal behavior
- **Process Spawning:** TermiEmu spawns shell processes with inherited environment
- **Signal Handling:** Careful signal forwarding is required for security

We continuously work to mitigate these risks through careful implementation and testing.

## Security Hall of Fame

We gratefully acknowledge security researchers who responsibly disclose vulnerabilities:

*(No vulnerabilities reported yet)*

---

## Questions?

If you have questions about this security policy, please open a [Discussion](https://github.com/merenut/TermiEmu/discussions) (for general questions) or use the reporting methods above (for security concerns).

## Policy Updates

This security policy may be updated periodically. Check the [latest version](https://github.com/merenut/TermiEmu/blob/main/SECURITY.md) for current information.

---

**Last Updated:** November 2, 2024  
**Version:** 1.0
