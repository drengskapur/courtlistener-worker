# Security Policy

## Supported Versions

We actively support the latest stable version of this project. Security updates are provided for:

| Version | Supported          |
| ------- | ------------------ |
| Latest  | :white_check_mark: |
| < Latest| :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via one of the following methods:

- **Email**: Open a security advisory at https://github.com/drengskapur/courtlistener-worker/security/advisories/new
- **Private**: Use GitHub's private vulnerability reporting feature

Please include the following information in your report:

- Type of vulnerability
- Full paths of source file(s) related to the vulnerability
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit the issue

We will acknowledge receipt of your vulnerability report and work with you to understand and resolve the issue quickly.

## Disclosure Policy

- We will acknowledge your report within 48 hours
- We will provide a more detailed response within 7 days indicating the next steps
- We will keep you informed of the progress towards fixing the vulnerability
- We will notify you when the vulnerability has been fixed

## Security Best Practices

When using this worker:

- **Never commit API tokens** to version control
- Use Wrangler secrets for production deployments
- Set `CORS_ALLOWED_ORIGINS` to restrict access in production
- Keep dependencies up to date (we use Dependabot for automated updates)
- Review the [Security Considerations](README.md#security-considerations) section in the README

