CREATE SCHEMA public;

CREATE TYPE public.push_type AS ENUM ('fcm', 'apns', 'apns_sandbox', 'noop');

CREATE TABLE public.clients (
    id text not null,
    tenant_id uuid not null,
    token text not null,

    type public.push_type not null default 'noop',

    relay_url text not null,
    relay_id text not null,

    registered_at timestamptz not null default now(),

    primary key (id, token, tenant_id)
);
