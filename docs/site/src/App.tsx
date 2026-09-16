import { useEffect, useState } from 'react';
import { Badge, Button, Tooltip } from '@fluentui/react-components';
import { localeFromPath, localeHref, localeNames, locales, messages, type Locale } from './i18n';

const repositoryUrl = 'https://github.com/ChicoDotNet/FerrumWeave';
const issuesUrl = `${repositoryUrl}/issues`;
const command = `dotnet new install FerrumWeave.Sdk::0.1.0-alpha.1\ndotnet new console -lang Rust -n HelloFerrum\ncd HelloFerrum\ndotnet run`;
const heroUrl = `${import.meta.env.BASE_URL}hero/ferrumweave-readme-cover.png`;

type PillarKind = 'network' | 'shield' | 'weave' | 'layers' | 'open';

function PillarIcon({ kind }: { kind: PillarKind }) {
  const common = { fill: 'none', stroke: 'currentColor', strokeWidth: 1.8, strokeLinecap: 'round' as const, strokeLinejoin: 'round' as const };

  if (kind === 'shield') {
    return <svg viewBox="0 0 32 32" aria-hidden="true"><path {...common} d="M16 3 27 7v8c0 7-4.8 11.6-11 14-6.2-2.4-11-7-11-14V7l11-4Z"/><path {...common} d="m11 16 3.2 3.2L21 12.4"/></svg>;
  }
  if (kind === 'layers') {
    return <svg viewBox="0 0 32 32" aria-hidden="true"><path {...common} d="m16 4 12 6-12 6L4 10l12-6Z"/><path {...common} d="m4 16 12 6 12-6"/><path {...common} d="m4 22 12 6 12-6"/></svg>;
  }
  if (kind === 'open') {
    return <svg viewBox="0 0 32 32" aria-hidden="true"><circle {...common} cx="16" cy="16" r="12"/><path {...common} d="M10 20c2.2-5.6 9.8-7.1 13.9-2.8M9 10.5h7V4"/></svg>;
  }
  if (kind === 'weave') {
    return <svg viewBox="0 0 32 32" aria-hidden="true"><path {...common} d="M7 11c0-4 3-7 7-7h4c4 0 7 3 7 7s-3 7-7 7h-4c-4 0-7 3-7 7"/><path {...common} d="M25 21c0 4-3 7-7 7h-4c-4 0-7-3-7-7s3-7 7-7h4c4 0 7-3 7-7"/></svg>;
  }
  return <svg viewBox="0 0 32 32" aria-hidden="true"><circle {...common} cx="8" cy="16" r="3"/><circle {...common} cx="24" cy="8" r="3"/><circle {...common} cx="24" cy="24" r="3"/><path {...common} d="m10.7 14.7 10.5-5.3M10.7 17.3l10.5 5.3"/></svg>;
}

function VisionDiagram({ title, description }: { title: string; description: string }) {
  return (
    <svg className="vision-diagram" viewBox="0 0 760 370" role="img" aria-labelledby="vision-title vision-desc">
      <title id="vision-title">{title}</title>
      <desc id="vision-desc">{description}</desc>
      <defs>
        <marker id="arrow" markerWidth="9" markerHeight="9" refX="8" refY="4.5" orient="auto"><path d="M0,0 L9,4.5 L0,9 Z" className="diagram-arrow" /></marker>
      </defs>
      <g className="diagram-language">
        <rect x="40" y="35" width="130" height="58" rx="12"/><text x="105" y="70">C#</text>
        <rect x="215" y="35" width="130" height="58" rx="12"/><text x="280" y="70">F#</text>
        <rect x="390" y="35" width="130" height="58" rx="12"/><text x="455" y="70">Visual Basic</text>
      </g>
      <g className="diagram-rust">
        <rect x="565" y="35" width="150" height="58" rx="12"/><text x="640" y="70">Rust</text>
      </g>
      <g className="diagram-lines" markerEnd="url(#arrow)">
        <path d="M105 93 C105 145 250 145 330 180"/>
        <path d="M280 93 C280 135 330 150 365 180"/>
        <path d="M455 93 C455 135 430 150 405 180"/>
        <path d="M640 93 C640 145 515 150 440 180"/>
      </g>
      <g className="diagram-core">
        <rect x="250" y="180" width="260" height="58" rx="14"/><text x="380" y="215">Common Type System</text>
        <path d="M380 238v32" markerEnd="url(#arrow)"/>
        <rect x="270" y="275" width="220" height="52" rx="14"/><text x="380" y="307">CIL + Metadata</text>
        <path d="M380 327v22" markerEnd="url(#arrow)"/>
        <text className="diagram-clr" x="380" y="365">CLR</text>
      </g>
    </svg>
  );
}

function CodeBlock({ children }: { children: string }) {
  return <pre className="code-block"><code>{children}</code></pre>;
}

function LocalizedHeroTitle({ title }: { title: string }) {
  const parts = title.split('.NET');
  if (parts.length !== 2) return <h1>{title}</h1>;
  return <h1>{parts[0]}<span>.NET{parts[1]}</span></h1>;
}

export function App() {
  const locale = localeFromPath(window.location.pathname);
  const m = messages[locale];
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    document.documentElement.lang = locale;
    document.title = `FerrumWeave — ${m.heroTitle}`;
  }, [locale, m.heroTitle]);

  async function copyCommand() {
    await navigator.clipboard.writeText(command);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1800);
  }

  function changeLocale(nextLocale: Locale) {
    window.location.assign(localeHref(nextLocale, import.meta.env.BASE_URL));
  }

  return (
    <>
      <header className="topbar">
        <div className="container d-flex align-items-center justify-content-between py-3">
          <a className="brand-link" href="#top" aria-label="FerrumWeave home">Ferrum<span>Weave</span></a>
          <nav className="d-flex align-items-center gap-3" aria-label="Primary navigation">
            <a href="#vision">{m.navVision}</a>
            <a href="#milestone">{m.navFoundation}</a>
            <select
              aria-label={m.language}
              value={locale}
              onChange={(event) => changeLocale(event.target.value as Locale)}
              style={{ maxWidth: '11rem', padding: '0.4rem 0.55rem', borderRadius: '0.45rem' }}
            >
              {locales.map((item) => <option value={item} key={item}>{localeNames[item]}</option>)}
            </select>
            <Button as="a" appearance="primary" href={repositoryUrl}>GitHub</Button>
          </nav>
        </div>
      </header>

      <main id="top">
        <section className="hero-section">
          <div className="container py-5 py-lg-6">
            <div className="row align-items-center g-5">
              <div className="col-12 col-lg-6">
                <Badge appearance="outline" color="informative" className="status-badge">{m.status}</Badge>
                <LocalizedHeroTitle title={m.heroTitle} />
                <p className="hero-lead">{m.heroLead}</p>
                <div className="d-flex flex-wrap gap-3 mt-4">
                  <Button as="a" appearance="primary" size="large" href={repositoryUrl}>{m.exploreRepository}</Button>
                  <Button as="a" appearance="secondary" size="large" href="#milestone">{m.seeFoundation}</Button>
                </div>
                <div className="command-card mt-4" aria-label="Target dotnet command experience">
                  <div className="command-card__bar">
                    <span>{m.commandTitle}</span>
                    <Tooltip content={copied ? m.copied : m.copy} relationship="label">
                      <Button appearance="subtle" size="small" onClick={copyCommand}>{copied ? m.copied : m.copy}</Button>
                    </Tooltip>
                  </div>
                  <CodeBlock>{command}</CodeBlock>
                </div>
              </div>
              <div className="col-12 col-lg-6">
                <div className="hero-art-frame">
                  <img src={heroUrl} alt="FerrumWeave brand artwork with Ferrum Ox and woven interoperability motif" />
                </div>
              </div>
            </div>
          </div>
        </section>

        <section className="section surface" aria-labelledby="pillars-heading">
          <div className="container">
            <div className="section-heading">
              <p className="eyebrow">{m.whyEyebrow}</p>
              <h2 id="pillars-heading">{m.pillarsHeading}</h2>
              <p>{m.pillarsIntro}</p>
            </div>
            <div className="row g-4">
              {m.pillars.map((pillar) => (
                <div className="col-12 col-md-6 col-xl" key={pillar.title}>
                  <article className="pillar-card h-100">
                    <div className="pillar-icon"><PillarIcon kind={pillar.icon} /></div>
                    <h3>{pillar.title}</h3>
                    <p>{pillar.body}</p>
                  </article>
                </div>
              ))}
            </div>
          </div>
        </section>

        <section className="section" id="vision" aria-labelledby="vision-heading">
          <div className="container">
            <div className="row align-items-center g-5">
              <div className="col-12 col-lg-5">
                <p className="eyebrow">{m.visionEyebrow}</p>
                <h2 id="vision-heading">{m.visionHeading}</h2>
                <p>{m.visionBody}</p>
                <p className="callout">{m.integrate}</p>
              </div>
              <div className="col-12 col-lg-7">
                <div className="diagram-frame"><VisionDiagram title={m.diagramTitle} description={m.diagramDesc} /></div>
              </div>
            </div>
          </div>
        </section>

        <section className="section surface" aria-labelledby="not-heading">
          <div className="container">
            <div className="section-heading narrow">
              <p className="eyebrow">{m.clarityEyebrow}</p>
              <h2 id="not-heading">{m.notHeading}</h2>
            </div>
            <div className="row g-3 not-grid">
              {m.notItems.map(([title, body]) => (
                <div className="col-12 col-md-6" key={title}>
                  <article className="not-card"><span aria-hidden="true">×</span><div><h3>{title}</h3><p>{body}</p></div></article>
                </div>
              ))}
            </div>
          </div>
        </section>

        <section className="section milestone-section" id="milestone" aria-labelledby="milestone-heading">
          <div className="container">
            <div className="row g-5 align-items-center">
              <div className="col-12 col-lg-6">
                <p className="eyebrow">{m.milestoneEyebrow}</p>
                <h2 id="milestone-heading">{m.milestoneHeading}</h2>
                <p>{m.milestoneBody}</p>
                <div className="proof-flow" aria-label={m.proofAria}>
                  {m.proofSteps.map((step, index) => (
                    <div className="proof-step" key={step}><span>{index + 1}</span>{step}</div>
                  ))}
                </div>
              </div>
              <div className="col-12 col-lg-6">
                <CodeBlock>{`dotnet new console -lang Rust -n HelloFerrum\ncd HelloFerrum\ndotnet run`}</CodeBlock>
                <p className="evidence-note">{m.evidenceNote}</p>
              </div>
            </div>
          </div>
        </section>

        <section className="join-section" aria-labelledby="join-heading">
          <div className="container py-5">
            <div className="join-card">
              <div>
                <p className="eyebrow">{m.joinEyebrow}</p>
                <h2 id="join-heading">{m.joinHeading}</h2>
                <p>{m.joinBody}</p>
              </div>
              <div className="d-flex flex-wrap gap-3">
                <Button as="a" appearance="primary" size="large" href={repositoryUrl}>{m.viewRepository}</Button>
                <Button as="a" appearance="secondary" size="large" href={issuesUrl}>{m.exploreIssues}</Button>
              </div>
            </div>
          </div>
        </section>
      </main>

      <footer>
        <div className="container py-4">
          <div className="row g-3 align-items-start">
            <div className="col-12 col-lg-4"><strong>Ferrum<span>Weave</span></strong><p>{m.footerTagline}</p></div>
            <div className="col-12 col-lg-4"><p>{m.licensedUnder} <a href={`${repositoryUrl}/blob/main/LICENSE-MIT`}>MIT</a> {m.or} <a href={`${repositoryUrl}/blob/main/LICENSE-APACHE`}>Apache-2.0</a>, {m.atYourOption}</p></div>
            <div className="col-12 col-lg-4"><p>{m.independent}</p></div>
          </div>
        </div>
      </footer>
    </>
  );
}
