import { useState } from 'react';
import { Badge, Button, Tooltip } from '@fluentui/react-components';

const repositoryUrl = 'https://github.com/ChicoDotNet/FerrumWeave';
const issuesUrl = `${repositoryUrl}/issues`;
const command = `dotnet new rust -n HelloFerrum\ncd HelloFerrum\ndotnet run`;
const heroUrl = `${import.meta.env.BASE_URL}hero/ferrumweave-readme-cover.png`;

const pillars = [
  {
    title: 'Multi-language ecosystem',
    body: 'Rust should live beside C#, F#, Visual Basic, and other CLR languages — not behind a service boundary.',
    icon: 'network',
  },
  {
    title: 'Memory & Safety by Design',
    body: 'Keep ownership, borrowing, traits, lifetimes, and explicit error handling meaningful on the CLR target.',
    icon: 'shield',
  },
  {
    title: 'Seamless Interop',
    body: 'Treat assemblies, metadata, CTS types, project references, and packages as native interoperability contracts.',
    icon: 'weave',
  },
  {
    title: 'Built on .NET & CLR',
    body: 'Use the runtime, type system, MSBuild, NuGet, and tooling that already make multi-language .NET possible.',
    icon: 'layers',
  },
  {
    title: 'Open Source from Day One',
    body: 'Build in public, keep provenance clean, prefer upstream collaboration, and leave room for neutral governance.',
    icon: 'open',
  },
] as const;

function PillarIcon({ kind }: { kind: (typeof pillars)[number]['icon'] }) {
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

function VisionDiagram() {
  return (
    <svg className="vision-diagram" viewBox="0 0 760 370" role="img" aria-labelledby="vision-title vision-desc">
      <title id="vision-title">FerrumWeave language interoperability vision</title>
      <desc id="vision-desc">C#, F#, Visual Basic, and Rust converge through the Common Type System, CIL and metadata, then execute on the CLR.</desc>
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

export function App() {
  const [copied, setCopied] = useState(false);

  async function copyCommand() {
    await navigator.clipboard.writeText(command);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1800);
  }

  return (
    <>
      <header className="topbar">
        <div className="container d-flex align-items-center justify-content-between py-3">
          <a className="brand-link" href="#top" aria-label="FerrumWeave home">Ferrum<span>Weave</span></a>
          <nav className="d-flex align-items-center gap-3" aria-label="Primary navigation">
            <a href="#vision">Vision</a>
            <a href="#milestone">First proof</a>
            <Button as="a" appearance="primary" href={repositoryUrl}>GitHub</Button>
          </nav>
        </div>
      </header>

      <main id="top">
        <section className="hero-section">
          <div className="container py-5 py-lg-6">
            <div className="row align-items-center g-5">
              <div className="col-12 col-lg-6">
                <Badge appearance="outline" color="informative" className="status-badge">Pre-alpha · architectural discovery</Badge>
                <h1>Bringing Rust into the <span>.NET ecosystem.</span></h1>
                <p className="hero-lead">FerrumWeave aims to make Rust a first-class .NET language so organizations can introduce Rust&apos;s safety model into new and critical components without abandoning the software, libraries, languages, and operational knowledge they already have.</p>
                <div className="d-flex flex-wrap gap-3 mt-4">
                  <Button as="a" appearance="primary" size="large" href={repositoryUrl}>Explore the repository</Button>
                  <Button as="a" appearance="secondary" size="large" href="#milestone">See the first proof</Button>
                </div>
                <div className="command-card mt-4" aria-label="Target dotnet command experience">
                  <div className="command-card__bar">
                    <span>Target developer experience</span>
                    <Tooltip content={copied ? 'Copied' : 'Copy commands'} relationship="label">
                      <Button appearance="subtle" size="small" onClick={copyCommand}>{copied ? 'Copied ✓' : 'Copy'}</Button>
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
              <p className="eyebrow">Why FerrumWeave</p>
              <h2 id="pillars-heading">Preserve what works. Strengthen what comes next.</h2>
              <p>Rust does not need to replace the .NET estate to improve its next critical component.</p>
            </div>
            <div className="row g-4">
              {pillars.map((pillar) => (
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
                <p className="eyebrow">The vision</p>
                <h2 id="vision-heading">Different source languages. One runtime contract.</h2>
                <p>FerrumWeave is not a native FFI bridge. The goal is Rust source compiled toward CIL and .NET metadata, participating in the same Common Type System that lets existing CLR languages interoperate.</p>
                <p className="callout">Integrate before reinventing.</p>
              </div>
              <div className="col-12 col-lg-7">
                <div className="diagram-frame"><VisionDiagram /></div>
              </div>
            </div>
          </div>
        </section>

        <section className="section surface" aria-labelledby="not-heading">
          <div className="container">
            <div className="section-heading narrow">
              <p className="eyebrow">Clarity earns trust</p>
              <h2 id="not-heading">What FerrumWeave is not</h2>
            </div>
            <div className="row g-3 not-grid">
              {[
                ['Not a new Rust-like language', 'Rust should remain Rust. Do not reimplement its parser, type system, borrow checker, or language evolution in another compiler.'],
                ['Not an FFI wrapper', 'The destination is CIL, metadata, CTS and CLR interoperability — not P/Invoke around a native Rust DLL.'],
                ['Not a replacement for .NET', 'FerrumWeave exists because the .NET runtime, ecosystem and decades of working business software are valuable.'],
                ['Not a replacement for native Rust', 'A CLR target is another deployment and interoperability option, not an argument that every Rust workload belongs on .NET.'],
              ].map(([title, body]) => (
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
                <p className="eyebrow">The first proof</p>
                <h2 id="milestone-heading">One trustworthy vertical slice.</h2>
                <p>The first meaningful milestone is deliberately small: a real `.rsproj`, built and run through `dotnet`, producing a valid .NET assembly whose Rust code calls `System.Console.WriteLine` through the CLR.</p>
                <div className="proof-flow" aria-label="First proof compilation flow">
                  {['Rust source', '.rsproj', 'dotnet build / run', 'CIL + metadata', 'CLR', 'System.Console.WriteLine'].map((step, index) => (
                    <div className="proof-step" key={step}><span>{index + 1}</span>{step}</div>
                  ))}
                </div>
              </div>
              <div className="col-12 col-lg-6">
                <CodeBlock>{`use dotnet::System::*;\n\nfn main() -> Result<()> {\n    Console::WriteLine("Hello from FerrumWeave")?;\n    Ok(())\n}`}</CodeBlock>
                <p className="evidence-note">Until this contract works end-to-end, the site will not pretend FerrumWeave is a finished language implementation.</p>
              </div>
            </div>
          </div>
        </section>

        <section className="join-section" aria-labelledby="join-heading">
          <div className="container py-5">
            <div className="join-card">
              <div>
                <p className="eyebrow">Open source from day one</p>
                <h2 id="join-heading">Help forge the first contract.</h2>
                <p>FerrumWeave is starting with the problem, principles, and executable contracts before claiming breadth. Compiler, CLR, Rust, .NET SDK, tooling, documentation, and testing experience are all useful here.</p>
              </div>
              <div className="d-flex flex-wrap gap-3">
                <Button as="a" appearance="primary" size="large" href={repositoryUrl}>View repository</Button>
                <Button as="a" appearance="secondary" size="large" href={issuesUrl}>Explore issues</Button>
              </div>
            </div>
          </div>
        </section>
      </main>

      <footer>
        <div className="container py-4">
          <div className="row g-3 align-items-start">
            <div className="col-12 col-lg-4"><strong>Ferrum<span>Weave</span></strong><p>Forge. Weave. Interoperate.</p></div>
            <div className="col-12 col-lg-4"><p>Licensed under <a href={`${repositoryUrl}/blob/main/LICENSE-MIT`}>MIT</a> or <a href={`${repositoryUrl}/blob/main/LICENSE-APACHE`}>Apache-2.0</a>, at your option.</p></div>
            <div className="col-12 col-lg-4"><p>Independent experimental project. Not affiliated with, sponsored by, or endorsed by Microsoft, the .NET Foundation, the Rust Foundation, or the Rust Project.</p></div>
          </div>
        </div>
      </footer>
    </>
  );
}
