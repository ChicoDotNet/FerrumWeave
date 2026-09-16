import { useCallback, useEffect, useState } from 'react';
import { locales, messages, type Locale } from './i18n';

const handoffParameter = 'route';

function isLocale(value: string | null): value is Locale {
  return value !== null && (locales as readonly string[]).includes(value);
}

function basePath(): string {
  return import.meta.env.BASE_URL.endsWith('/') ? import.meta.env.BASE_URL : `${import.meta.env.BASE_URL}/`;
}

function localePath(locale: Locale): string {
  return locale === 'en' ? basePath() : `${basePath()}${locale}/`;
}

function localeFromPathname(pathname: string): Locale {
  const base = basePath();
  const relative = pathname.startsWith(base) ? pathname.slice(base.length) : pathname.replace(/^\/+/, '');
  const segment = relative.split('/').filter(Boolean)[0] ?? null;
  return isLocale(segment) ? segment : 'en';
}

function initialLocale(): Locale {
  const handoff = new URLSearchParams(window.location.search).get(handoffParameter);
  return isLocale(handoff) ? handoff : localeFromPathname(window.location.pathname);
}

function absoluteUrl(path: string): string {
  return new URL(path, window.location.origin).href;
}

function ensureLink(rel: string, hreflang?: string): HTMLLinkElement {
  const selector = hreflang
    ? `link[rel="${rel}"][hreflang="${hreflang}"]`
    : `link[rel="${rel}"]:not([hreflang])`;
  let link = document.head.querySelector<HTMLLinkElement>(selector);
  if (!link) {
    link = document.createElement('link');
    link.rel = rel;
    if (hreflang) link.hreflang = hreflang;
    document.head.appendChild(link);
  }
  return link;
}

function ensureMeta(name: string): HTMLMetaElement {
  let meta = document.head.querySelector<HTMLMetaElement>(`meta[name="${name}"]`);
  if (!meta) {
    meta = document.createElement('meta');
    meta.name = name;
    document.head.appendChild(meta);
  }
  return meta;
}

function applyMetadata(locale: Locale): void {
  const copy = messages[locale];
  document.documentElement.lang = locale;
  document.title = `FerrumWeave — ${copy.heroTitle}`;
  ensureMeta('description').content = copy.heroLead;

  ensureLink('canonical').href = absoluteUrl(localePath(locale));
  for (const alternateLocale of locales) {
    ensureLink('alternate', alternateLocale).href = absoluteUrl(localePath(alternateLocale));
  }
  ensureLink('alternate', 'x-default').href = absoluteUrl(localePath('en'));
}

export function useSiteRuntime(): { locale: Locale; navigateLocale: (locale: Locale) => void } {
  const [locale, setLocale] = useState<Locale>(initialLocale);

  useEffect(() => {
    const handoff = new URLSearchParams(window.location.search).get(handoffParameter);
    if (isLocale(handoff)) {
      window.history.replaceState({}, '', localePath(handoff));
    }

    const onPopState = () => setLocale(localeFromPathname(window.location.pathname));
    window.addEventListener('popstate', onPopState);
    return () => window.removeEventListener('popstate', onPopState);
  }, []);

  useEffect(() => {
    applyMetadata(locale);
  }, [locale]);

  const navigateLocale = useCallback((nextLocale: Locale) => {
    if (nextLocale === locale) return;
    window.history.pushState({}, '', localePath(nextLocale));
    setLocale(nextLocale);
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }, [locale]);

  return { locale, navigateLocale };
}
