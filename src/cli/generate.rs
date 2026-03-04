use anyhow::{Context, Result};
use colored::Colorize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::crypto;
use crate::env_parser;
use crate::vault;

pub fn handle_generate(template_path: &str) -> Result<()> {
    let path = Path::new(template_path);

    // Check built-in templates first
    let template_content = if !path.exists() {
        // Try as a built-in template name
        match get_builtin_template(template_path) {
            Some(content) => content,
            None => {
                println!(
                    "{} Template not found: {}",
                    "Error:".red(),
                    template_path
                );
                println!();
                println!("Built-in templates:");
                println!();
                println!("  -- JavaScript / TypeScript --");
                println!("  nextjs      Next.js application");
                println!("  nextjs-full Next.js (full: auth, DB, email, storage)");
                println!("  t3          T3 Stack (Next.js + Prisma + tRPC)");
                println!("  nuxtjs      Nuxt 3 application");
                println!("  sveltekit   SvelteKit application");
                println!("  remix       Remix application");
                println!("  astro       Astro site");
                println!("  vite        Vite generic app");
                println!("  react       Create React App / Vite React");
                println!("  express     Express.js API");
                println!("  nestjs      NestJS application");
                println!("  graphql     GraphQL server (Apollo/Yoga)");
                println!();
                println!("  -- Python --");
                println!("  django      Django application");
                println!("  fastapi     FastAPI application");
                println!("  flask       Flask application");
                println!();
                println!("  -- Other languages --");
                println!("  rails       Ruby on Rails");
                println!("  laravel     Laravel (PHP)");
                println!("  golang      Go application");
                println!("  rust        Rust application");
                println!("  elixir      Elixir / Phoenix");
                println!();
                println!("  -- Infrastructure & databases --");
                println!("  docker      Docker Compose environment");
                println!("  postgres    PostgreSQL standalone");
                println!("  mongodb     MongoDB");
                println!("  redis       Redis standalone");
                println!("  prisma      Prisma ORM");
                println!("  hasura      Hasura GraphQL Engine");
                println!();
                println!("  -- Cloud & services --");
                println!("  supabase    Supabase");
                println!("  firebase    Firebase");
                println!("  aws         AWS credentials & config");
                println!("  gcp         Google Cloud Platform");
                println!("  azure       Microsoft Azure");
                println!();
                println!("  -- Integrations --");
                println!("  stripe      Stripe payments");
                println!("  sendgrid    SendGrid email");
                println!("  twilio      Twilio SMS / Voice");
                println!("  sentry      Sentry error tracking");
                println!("  oauth       OAuth / Social auth");
                return Ok(());
            }
        }
    } else {
        fs::read_to_string(path)
            .map_err(|e| crate::errors::EnvkeepError::FileReadError(
                template_path.to_string(), e
            ))?
    };

    // Parse the template
    let template_vars = parse_template(&template_content);

    // Try to fill from vault if available
    let mut filled_vars = BTreeMap::new();
    let mut from_vault = 0;
    let mut from_default = 0;
    let mut empty = 0;

    let vault_vars = try_load_vault_vars();

    for (key, default_value) in &template_vars {
        if let Some(vault_value) = vault_vars.as_ref().and_then(|v| v.get(key)) {
            filled_vars.insert(key.clone(), vault_value.clone());
            from_vault += 1;
        } else if !default_value.is_empty() {
            filled_vars.insert(key.clone(), default_value.clone());
            from_default += 1;
        } else {
            filled_vars.insert(key.clone(), String::new());
            empty += 1;
        }
    }

    // Write the .env file
    let output_path = Path::new(".env");
    env_parser::write_env_file(output_path, &filled_vars)?;

    println!(
        "{} Generated .env with {} variables",
        "Done.".green().bold(),
        template_vars.len()
    );
    println!(
        "  {} from vault, {} defaults, {} empty (fill manually)",
        from_vault, from_default, empty
    );

    Ok(())
}

/// Parse a template file into key-default pairs.
fn parse_template(content: &str) -> BTreeMap<String, String> {
    let mut vars = BTreeMap::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim().to_string();
            let value = trimmed[eq_pos + 1..].trim().to_string();
            if !key.is_empty() {
                vars.insert(key, value);
            }
        }
    }

    vars
}

fn get_builtin_template(name: &str) -> Option<String> {
    match name {
        // ----------------------------------------------------------------
        // JavaScript / TypeScript
        // ----------------------------------------------------------------
        "nextjs" => Some(
            "# Next.js\n\
             NEXT_PUBLIC_APP_URL=http://localhost:3000\n\
             NEXT_PUBLIC_API_URL=\n\
             DATABASE_URL=\n\
             NEXTAUTH_SECRET=\n\
             NEXTAUTH_URL=http://localhost:3000\n"
                .to_string(),
        ),
        "nextjs-full" => Some(
            "# Next.js (full stack)\n\
             NEXT_PUBLIC_APP_URL=http://localhost:3000\n\
             NEXT_PUBLIC_API_URL=\n\
             DATABASE_URL=\n\
             NEXTAUTH_SECRET=\n\
             NEXTAUTH_URL=http://localhost:3000\n\
             # Email (Resend / SMTP)\n\
             SMTP_HOST=\n\
             SMTP_PORT=587\n\
             SMTP_USER=\n\
             SMTP_PASSWORD=\n\
             EMAIL_FROM=noreply@example.com\n\
             # Object storage (S3-compatible)\n\
             STORAGE_BUCKET=\n\
             STORAGE_REGION=us-east-1\n\
             STORAGE_ENDPOINT=\n\
             STORAGE_ACCESS_KEY=\n\
             STORAGE_SECRET_KEY=\n\
             # Analytics\n\
             NEXT_PUBLIC_GA_ID=\n\
             NEXT_PUBLIC_POSTHOG_KEY=\n"
                .to_string(),
        ),
        "t3" => Some(
            "# T3 Stack (Next.js + Prisma + tRPC + NextAuth)\n\
             DATABASE_URL=\n\
             NEXTAUTH_SECRET=\n\
             NEXTAUTH_URL=http://localhost:3000\n\
             # OAuth providers (add as needed)\n\
             DISCORD_CLIENT_ID=\n\
             DISCORD_CLIENT_SECRET=\n\
             GITHUB_CLIENT_ID=\n\
             GITHUB_CLIENT_SECRET=\n"
                .to_string(),
        ),
        "nuxtjs" => Some(
            "# Nuxt 3\n\
             NUXT_PUBLIC_BASE_URL=http://localhost:3000\n\
             NUXT_SECRET=\n\
             DATABASE_URL=\n\
             NUXT_PUBLIC_API_BASE=\n"
                .to_string(),
        ),
        "sveltekit" => Some(
            "# SvelteKit\n\
             ORIGIN=http://localhost:5173\n\
             DATABASE_URL=\n\
             JWT_SECRET=\n\
             PUBLIC_API_URL=http://localhost:5173\n"
                .to_string(),
        ),
        "remix" => Some(
            "# Remix\n\
             SESSION_SECRET=\n\
             DATABASE_URL=\n\
             NODE_ENV=development\n\
             PORT=3000\n"
                .to_string(),
        ),
        "astro" => Some(
            "# Astro\n\
             SITE=http://localhost:4321\n\
             PUBLIC_API_URL=\n\
             DATABASE_URL=\n\
             SECRET_KEY=\n"
                .to_string(),
        ),
        "vite" => Some(
            "# Vite\n\
             VITE_API_URL=http://localhost:3000\n\
             VITE_APP_TITLE=My App\n\
             VITE_PUBLIC_KEY=\n"
                .to_string(),
        ),
        "react" => Some(
            "# React (Create React App / Vite)\n\
             REACT_APP_API_URL=http://localhost:3000\n\
             REACT_APP_ENV=development\n\
             REACT_APP_PUBLIC_KEY=\n"
                .to_string(),
        ),
        "express" => Some(
            "# Express.js API\n\
             PORT=3000\n\
             NODE_ENV=development\n\
             DATABASE_URL=\n\
             JWT_SECRET=\n\
             JWT_EXPIRES_IN=7d\n\
             CORS_ORIGIN=http://localhost:3000\n\
             RATE_LIMIT_WINDOW_MS=900000\n\
             RATE_LIMIT_MAX=100\n"
                .to_string(),
        ),
        "nestjs" => Some(
            "# NestJS\n\
             PORT=3000\n\
             NODE_ENV=development\n\
             DATABASE_URL=\n\
             JWT_SECRET=\n\
             JWT_EXPIRES_IN=7d\n\
             REDIS_URL=redis://localhost:6379\n\
             THROTTLE_TTL=60\n\
             THROTTLE_LIMIT=100\n"
                .to_string(),
        ),
        "graphql" => Some(
            "# GraphQL server (Apollo / Yoga)\n\
             PORT=4000\n\
             NODE_ENV=development\n\
             DATABASE_URL=\n\
             JWT_SECRET=\n\
             INTROSPECTION=true\n\
             PLAYGROUND=true\n\
             CORS_ORIGIN=http://localhost:3000\n"
                .to_string(),
        ),
        // ----------------------------------------------------------------
        // Python
        // ----------------------------------------------------------------
        "django" => Some(
            "# Django\n\
             DJANGO_SECRET_KEY=\n\
             DJANGO_DEBUG=True\n\
             DJANGO_ALLOWED_HOSTS=localhost,127.0.0.1\n\
             DATABASE_URL=postgres://localhost:5432/mydb\n\
             REDIS_URL=redis://localhost:6379/0\n\
             EMAIL_HOST=\n\
             EMAIL_PORT=587\n\
             EMAIL_HOST_USER=\n\
             EMAIL_HOST_PASSWORD=\n\
             CORS_ALLOWED_ORIGINS=http://localhost:3000\n"
                .to_string(),
        ),
        "fastapi" => Some(
            "# FastAPI\n\
             APP_ENV=development\n\
             DEBUG=true\n\
             HOST=0.0.0.0\n\
             PORT=8000\n\
             DATABASE_URL=postgresql+asyncpg://user:password@localhost:5432/mydb\n\
             REDIS_URL=redis://localhost:6379\n\
             SECRET_KEY=\n\
             ACCESS_TOKEN_EXPIRE_MINUTES=30\n\
             CORS_ORIGINS=http://localhost:3000\n"
                .to_string(),
        ),
        "flask" => Some(
            "# Flask\n\
             FLASK_APP=app.py\n\
             FLASK_ENV=development\n\
             SECRET_KEY=\n\
             DATABASE_URL=sqlite:///app.db\n\
             REDIS_URL=redis://localhost:6379\n\
             MAIL_SERVER=\n\
             MAIL_PORT=587\n\
             MAIL_USERNAME=\n\
             MAIL_PASSWORD=\n"
                .to_string(),
        ),
        // ----------------------------------------------------------------
        // Other languages
        // ----------------------------------------------------------------
        "rails" => Some(
            "# Ruby on Rails\n\
             RAILS_ENV=development\n\
             SECRET_KEY_BASE=\n\
             DATABASE_URL=postgres://localhost:5432/myapp_development\n\
             REDIS_URL=redis://localhost:6379/0\n\
             RAILS_MASTER_KEY=\n\
             RAILS_LOG_TO_STDOUT=true\n\
             RAILS_SERVE_STATIC_FILES=false\n"
                .to_string(),
        ),
        "laravel" => Some(
            "# Laravel\n\
             APP_NAME=Laravel\n\
             APP_ENV=local\n\
             APP_KEY=\n\
             APP_DEBUG=true\n\
             APP_URL=http://localhost\n\
             DB_CONNECTION=mysql\n\
             DB_HOST=127.0.0.1\n\
             DB_PORT=3306\n\
             DB_DATABASE=laravel\n\
             DB_USERNAME=root\n\
             DB_PASSWORD=\n\
             CACHE_DRIVER=file\n\
             SESSION_DRIVER=file\n\
             QUEUE_CONNECTION=sync\n\
             MAIL_MAILER=smtp\n\
             MAIL_HOST=mailpit\n\
             MAIL_PORT=1025\n\
             MAIL_USERNAME=\n\
             MAIL_PASSWORD=\n"
                .to_string(),
        ),
        "golang" => Some(
            "# Go application\n\
             APP_ENV=development\n\
             SERVER_PORT=8080\n\
             DATABASE_DSN=postgres://user:password@localhost:5432/mydb?sslmode=disable\n\
             REDIS_ADDR=localhost:6379\n\
             JWT_SECRET=\n\
             LOG_LEVEL=info\n\
             CORS_ALLOWED_ORIGINS=http://localhost:3000\n"
                .to_string(),
        ),
        "rust" => Some(
            "# Rust application\n\
             APP_ENV=development\n\
             SERVER_HOST=0.0.0.0\n\
             SERVER_PORT=8080\n\
             DATABASE_URL=postgres://user:password@localhost:5432/mydb\n\
             REDIS_URL=redis://localhost:6379\n\
             JWT_SECRET=\n\
             RUST_LOG=info\n"
                .to_string(),
        ),
        "elixir" => Some(
            "# Elixir / Phoenix\n\
             MIX_ENV=dev\n\
             PHX_HOST=localhost\n\
             PHX_SECRET_KEY_BASE=\n\
             DATABASE_URL=ecto://postgres:postgres@localhost/myapp_dev\n\
             PORT=4000\n\
             POOL_SIZE=10\n"
                .to_string(),
        ),
        // ----------------------------------------------------------------
        // Infrastructure & databases
        // ----------------------------------------------------------------
        "docker" => Some(
            "# Docker Compose\n\
             COMPOSE_PROJECT_NAME=\n\
             POSTGRES_USER=postgres\n\
             POSTGRES_PASSWORD=\n\
             POSTGRES_DB=app\n\
             POSTGRES_PORT=5432\n\
             REDIS_URL=redis://redis:6379\n\
             REDIS_PORT=6379\n\
             APP_PORT=3000\n"
                .to_string(),
        ),
        "postgres" => Some(
            "# PostgreSQL\n\
             POSTGRES_HOST=localhost\n\
             POSTGRES_PORT=5432\n\
             POSTGRES_USER=postgres\n\
             POSTGRES_PASSWORD=\n\
             POSTGRES_DB=mydb\n\
             POSTGRES_SSL_MODE=disable\n\
             POSTGRES_MAX_CONNECTIONS=10\n"
                .to_string(),
        ),
        "mongodb" => Some(
            "# MongoDB\n\
             MONGODB_URI=mongodb://localhost:27017/mydb\n\
             MONGODB_USER=\n\
             MONGODB_PASSWORD=\n\
             MONGODB_DB=mydb\n\
             MONGODB_AUTH_SOURCE=admin\n"
                .to_string(),
        ),
        "redis" => Some(
            "# Redis\n\
             REDIS_HOST=localhost\n\
             REDIS_PORT=6379\n\
             REDIS_PASSWORD=\n\
             REDIS_DB=0\n\
             REDIS_TLS=false\n\
             REDIS_MAX_RETRIES=3\n"
                .to_string(),
        ),
        "prisma" => Some(
            "# Prisma ORM\n\
             DATABASE_URL=postgresql://user:password@localhost:5432/mydb?schema=public\n\
             SHADOW_DATABASE_URL=\n"
                .to_string(),
        ),
        "hasura" => Some(
            "# Hasura GraphQL Engine\n\
             HASURA_GRAPHQL_DATABASE_URL=postgres://user:password@localhost:5432/mydb\n\
             HASURA_GRAPHQL_ADMIN_SECRET=\n\
             HASURA_GRAPHQL_JWT_SECRET=\n\
             HASURA_GRAPHQL_ENABLE_CONSOLE=true\n\
             HASURA_GRAPHQL_DEV_MODE=true\n\
             HASURA_GRAPHQL_ENABLED_LOG_TYPES=startup,http-log,webhook-log,websocket-log\n"
                .to_string(),
        ),
        // ----------------------------------------------------------------
        // Cloud & services
        // ----------------------------------------------------------------
        "supabase" => Some(
            "# Supabase\n\
             NEXT_PUBLIC_SUPABASE_URL=\n\
             NEXT_PUBLIC_SUPABASE_ANON_KEY=\n\
             SUPABASE_SERVICE_ROLE_KEY=\n\
             SUPABASE_JWT_SECRET=\n\
             DATABASE_URL=\n"
                .to_string(),
        ),
        "firebase" => Some(
            "# Firebase\n\
             NEXT_PUBLIC_FIREBASE_API_KEY=\n\
             NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=\n\
             NEXT_PUBLIC_FIREBASE_PROJECT_ID=\n\
             NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET=\n\
             NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID=\n\
             NEXT_PUBLIC_FIREBASE_APP_ID=\n\
             FIREBASE_ADMIN_CLIENT_EMAIL=\n\
             FIREBASE_ADMIN_PRIVATE_KEY=\n"
                .to_string(),
        ),
        "aws" => Some(
            "# AWS\n\
             AWS_ACCESS_KEY_ID=\n\
             AWS_SECRET_ACCESS_KEY=\n\
             AWS_SESSION_TOKEN=\n\
             AWS_DEFAULT_REGION=us-east-1\n\
             AWS_S3_BUCKET=\n\
             AWS_CLOUDFRONT_URL=\n\
             AWS_SES_REGION=us-east-1\n"
                .to_string(),
        ),
        "gcp" => Some(
            "# Google Cloud Platform\n\
             GOOGLE_CLOUD_PROJECT=\n\
             GOOGLE_APPLICATION_CREDENTIALS=\n\
             GCS_BUCKET=\n\
             GOOGLE_CLIENT_ID=\n\
             GOOGLE_CLIENT_SECRET=\n"
                .to_string(),
        ),
        "azure" => Some(
            "# Microsoft Azure\n\
             AZURE_TENANT_ID=\n\
             AZURE_CLIENT_ID=\n\
             AZURE_CLIENT_SECRET=\n\
             AZURE_SUBSCRIPTION_ID=\n\
             AZURE_STORAGE_ACCOUNT=\n\
             AZURE_STORAGE_KEY=\n\
             AZURE_STORAGE_CONNECTION_STRING=\n"
                .to_string(),
        ),
        // ----------------------------------------------------------------
        // Integrations
        // ----------------------------------------------------------------
        "stripe" => Some(
            "# Stripe\n\
             STRIPE_PUBLISHABLE_KEY=pk_test_\n\
             STRIPE_SECRET_KEY=sk_test_\n\
             STRIPE_WEBHOOK_SECRET=whsec_\n\
             STRIPE_PRICE_ID=\n\
             NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY=pk_test_\n"
                .to_string(),
        ),
        "sendgrid" => Some(
            "# SendGrid\n\
             SENDGRID_API_KEY=\n\
             SENDGRID_FROM_EMAIL=noreply@example.com\n\
             SENDGRID_FROM_NAME=My App\n\
             SENDGRID_WELCOME_TEMPLATE_ID=\n\
             SENDGRID_PASSWORD_RESET_TEMPLATE_ID=\n"
                .to_string(),
        ),
        "twilio" => Some(
            "# Twilio\n\
             TWILIO_ACCOUNT_SID=\n\
             TWILIO_AUTH_TOKEN=\n\
             TWILIO_PHONE_NUMBER=\n\
             TWILIO_MESSAGING_SERVICE_SID=\n"
                .to_string(),
        ),
        "sentry" => Some(
            "# Sentry\n\
             SENTRY_DSN=\n\
             SENTRY_ORG=\n\
             SENTRY_PROJECT=\n\
             SENTRY_AUTH_TOKEN=\n\
             NEXT_PUBLIC_SENTRY_DSN=\n\
             SENTRY_ENVIRONMENT=development\n\
             SENTRY_TRACES_SAMPLE_RATE=0.1\n"
                .to_string(),
        ),
        "oauth" => Some(
            "# OAuth / Social auth\n\
             GITHUB_CLIENT_ID=\n\
             GITHUB_CLIENT_SECRET=\n\
             GOOGLE_CLIENT_ID=\n\
             GOOGLE_CLIENT_SECRET=\n\
             DISCORD_CLIENT_ID=\n\
             DISCORD_CLIENT_SECRET=\n\
             TWITTER_CLIENT_ID=\n\
             TWITTER_CLIENT_SECRET=\n\
             LINKEDIN_CLIENT_ID=\n\
             LINKEDIN_CLIENT_SECRET=\n\
             OAUTH_CALLBACK_URL=http://localhost:3000/auth/callback\n"
                .to_string(),
        ),
        _ => None,
    }
}

/// Try to load variables from the most recently used project in the vault.
fn try_load_vault_vars() -> Option<BTreeMap<String, String>> {
    // This is a best-effort operation -- if vault is locked or empty, return None.
    // For now, return None. A more complete implementation would prompt for
    // a password and load the most recent project.
    None
}