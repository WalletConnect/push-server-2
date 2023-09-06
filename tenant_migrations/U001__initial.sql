CREATE SCHEMA public;

CREATE TABLE public.tenants (
    id uuid not null primary key default gen_random_uuid(),

    suspended bool not null default false,
    suspended_reason text,

    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

CREATE TYPE public.tenant_credential_type AS ENUM ('apns_token', 'apns_certificate', 'fcm_key', 'fcm_file');

CREATE TABLE public.tenant_credentials (
    id uuid not null primary key default gen_random_uuid(),
    tenant_id uuid not null references public.tenants(id),

    type public.tenant_credential_type not null,

    file text,
    values jsonb not null default '{}'::jsonb
);
