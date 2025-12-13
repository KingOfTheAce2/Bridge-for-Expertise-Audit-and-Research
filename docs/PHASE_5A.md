## Phase 5a: Licensing & Payment System (Priority: HIGH)
**Revenue Model - Supporting Social Mission**

### Philosophy: Local-First, Value-Based Tiers

This is a **local, single-user desktop application**. The user provides all compute, storage, and resources. Therefore, tier restrictions should be based on **value we provide**, not artificial limits on local functionality.

**Guiding Principles**:
1. **Never restrict local resources** - If it runs on their hardware, they can use it
2. **Charge for curated content** - Law libraries, professional templates cost money to create/license
3. **Charge for support** - Human time has real cost
4. **Charge for early access** - Development effort has value
5. **Keep free tier fully functional** - Social mission first

---

### Step 24: License Tier System
**Priority**: Critical | **Effort**: High | **Legal Risk**: Medium

**What**: Implement tiered licensing with local validation and payment integration (Stripe, Mollie, or Plug and Play).

---

### Pricing Tiers

#### 🆓 **Free Tier** (Community Edition)
**Target**: Everyone - individuals, students, small practices, social legal institutes
**Price**: €0 / Free forever

**Full Local Functionality**:
- ✅ Complete chat interface with all features
- ✅ **Unlimited local AI inference** (any model your hardware supports)
- ✅ **Unlimited system prompts** (create, edit, organize)
- ✅ **Unlimited local RAG** (as many documents as your storage allows)
- ✅ **Full PII detection** (Layer 1 regex + Layer 2 NER + Layer 3 Presidio)
- ✅ **All export formats** (PDF, DOCX, TXT, Markdown)
- ✅ GDPR & AI Act compliant
- ✅ All 5 languages (EN/NL/DE/FR/ZH)
- ✅ Dark mode and all UI features
- ✅ Case/matter organization
- ✅ Full prompt library (create your own)
- ✅ Document templates (create your own)
- ✅ Audit logging and compliance features

**What's NOT included**:
- ❌ Curated Law Library RAG (licensed legal databases)
- ❌ Professional template pack (lawyer-drafted templates)
- ❌ Premium system prompts (expert-crafted)
- ❌ Priority support
- ❌ Early access to new features

**No verification required** - just download and use.

---

#### 💼 **Professional Tier**
**Target**: Solo practitioners, freelance lawyers, small firms wanting curated content
**Price**: €14.99/month or €149/year (save 17%)
**USD**: $14.99/month or $149/year

**Everything in Free, plus**:
- ✅ **Professional Template Pack** (50+ lawyer-drafted templates)
  - Contracts (NDA, SLA, employment, consulting)
  - Privacy policies (GDPR-compliant, multi-jurisdiction)
  - Data Processing Agreements
  - Terms of Service templates
  - Client intake forms
  - Legal letters and notices
  - Due diligence checklists
- ✅ **Premium System Prompts** (20+ expert-crafted prompts)
  - Contract risk analysis
  - GDPR compliance checker
  - Legal research frameworks
  - Citation verification
  - Clause comparison
  - Jurisdiction-specific prompts (NL/BE/DE/FR)
- ✅ **Email support** (48h response time)
- ✅ **Early access** to new features (2 weeks before public)
- ✅ **Template updates** (new templates added monthly)

---

#### 🏆 **Pro+ Tier** (Law Library)
**Target**: Lawyers needing legal research capabilities
**Price**: €29.99/month or €299/year (save 17%)
**USD**: $29.99/month or $299/year

**Everything in Professional, plus**:
- ✅ **Law Library RAG** (searchable legal databases - local cache)
  - GDPR full text + recitals + case law summaries
  - EU regulations and directives (key texts)
  - Dutch law collection (BW, Wbp successor, key statutes)
  - Belgian law collection (key civil/commercial)
  - German law collection (BGB, DSGVO, key statutes)
  - French law collection (Code civil, RGPD, key statutes)
  - Case law summaries (CJEU, national high courts)
  - Legal commentary excerpts (where licensed)
- ✅ **Citation verification** (check if citations exist)
- ✅ **Precedent finder** (find relevant case law)
- ✅ **Cross-reference tool** (link related provisions)
- ✅ **Priority email support** (24h response time)
- ✅ **Quarterly law library updates**

**Note**: Law Library is cached locally after download. Works offline after initial sync.

---

#### 🏢 **Team Tier**
**Target**: Law firms, legal departments, multi-user teams
**Price**: €24.99/month per seat (minimum 3 seats)
**USD**: $24.99/month per seat

**Everything in Pro+, plus**:
- ✅ **Centralized billing** (one invoice for all seats)
- ✅ **Shared template library** (team templates synced locally)
- ✅ **Shared prompt library** (team prompts synced locally)
- ✅ **License management portal** (add/remove seats)
- ✅ **Dedicated support** (priority + video calls)
- ✅ **Custom onboarding session** (1 hour)
- ✅ **Quarterly review calls** (optional)

**Volume discounts**:
- 10+ seats: 10% discount
- 25+ seats: 15% discount
- 50+ seats: 20% discount
- 100+ seats: Contact for custom pricing

**Technical Note**: Team sync uses encrypted local network or manual export/import. No cloud storage of user data.

---

### What's Never Restricted (Any Tier)

| Feature | Why It's Free |
|---------|---------------|
| Model size (7B, 13B, 70B, etc.) | Runs on user's hardware |
| Number of documents in RAG | User's storage |
| Number of prompts | Just text files |
| Number of conversations | User's database |
| PII detection layers | Core privacy feature |
| Export formats | Basic functionality |
| Languages | Already built in |
| Offline usage | Local app by design |

---

### What's Paid (And Why)

| Feature | Tier | Why It Costs |
|---------|------|--------------|
| Professional templates | Professional+ | Lawyer time to draft |
| Premium prompts | Professional+ | Expert time to craft |
| Law Library | Pro+ | Licensing fees, curation effort |
| Priority support | Professional+ | Human time |
| Team sync | Team | Server infrastructure |
| Custom onboarding | Team | Human time |

---

### Payment Gateway Integration

**Supported Payment Providers**:

1. **Stripe** (Primary - Global)
   - Credit/debit cards
   - SEPA Direct Debit (Europe)
   - iDEAL (Netherlands)
   - Bancontact (Belgium)
   - Apple Pay / Google Pay
   - Subscription management
   - Invoice generation

2. **Mollie** (Alternative - Europe-focused)
   - All European payment methods
   - iDEAL, Bancontact, Sofort
   - SEPA Direct Debit
   - Credit cards
   - PayPal
   - Better European coverage

**Implementation Strategy**:
- **Online activation** (preferred): Payment → instant license key
- **Offline activation** (optional): Purchase → manual license key (for air-gapped setups)
- **License validation**: Local validation with periodic online check (monthly)
- **Grace period**: 30 days if offline or payment fails
- **Downgrade behavior**: Lose access to paid content, keep all local functionality

**Technical Flow**:
```
1. User selects tier → Payment page (Stripe/Mollie)
2. Payment successful → Generate license key
3. License key → Stored locally (encrypted)
4. Monthly validation check (non-blocking)
5. If validation fails → 30-day grace period
6. After grace → Downgrade (paid content locked, local features intact)
```

**License Key Format**:
```
Format: BEAR-XXXX-XXXX-XXXX-XXXX
Example: BEAR-PRO1-A3F9-K8L2-9X4M

Encoding:
- BEAR: Product identifier
- PRO1/PROP/TEAM: Tier identifier
- Next 3 blocks: Encrypted data (tier, expiry, features)
- Signed with RSA private key (verified locally)
```

---

### Social Mission: Discounts & Free Access

**For qualifying organizations, Professional tier is FREE**:

**Eligible Organizations**:
- Rechtswinkels (Legal Advice Centers - NL)
- Sociale advocatuur (Social Legal Aid - NL/BE)
- Juridisch Loket (Legal Counter - NL)
- Pro bono partnerships
- University legal clinics
- Refugee legal aid organizations
- Public interest law organizations
- Legal aid societies (international)

**Application Process**:
```
Apply → Submit proof → Review (3-5 business days) → Approved → Free Professional license
```

**Required Documentation**:
- Organization registration (KvK or equivalent)
- Organization email domain
- Brief description of legal aid work
- Annual renewal (simple re-confirmation)

**Student Discount**: 50% off Professional tier with valid .edu email

---

### Success Criteria

- ✅ Payment processing works in EUR and USD
- ✅ License activation completes in <30 seconds
- ✅ Offline activation works for air-gapped systems
- ✅ Free tier is fully functional for all local features
- ✅ Paid content (templates, law library) properly gated
- ✅ Downgrade preserves all local data and functionality
- ✅ Social mission application process works
- ✅ License renewal automatic (if enabled)
- ✅ Grace period prevents sudden lockout

---

### Rust Files (Licensing & Payment)

```
src-tauri/src/
├── licensing/
│   ├── mod.rs                           # License manager
│   ├── tier.rs                          # Tier definitions and features
│   ├── validator.rs                     # License key validation
│   ├── activation.rs                    # License activation
│   ├── verification.rs                  # Periodic verification
│   ├── grace_period.rs                  # Grace period management
│   └── content_access.rs                # Paid content access control
├── payment/
│   ├── mod.rs                           # Payment module
│   ├── stripe.rs                        # Stripe integration
│   ├── mollie.rs                        # Mollie integration
│   ├── webhook.rs                       # Payment webhooks
│   └── invoice.rs                       # Invoice generation
└── commands/
    ├── licensing.rs                     # License commands
    └── payment.rs                       # Payment commands

migration/src/
├── m20250116_000016_add_licenses.rs     # License table
└── m20250117_000017_add_subscriptions.rs # Subscription tracking

entity/src/
├── licenses.rs                          # License entity
└── subscriptions.rs                     # Subscription entity
```

---

### Privacy & Security

- ✅ **No telemetry**: Only license validation pings (once/month)
- ✅ **Encrypted license keys**: RSA-2048 signed
- ✅ **Local storage**: Payment info on Stripe/Mollie (not stored locally)
- ✅ **Offline mode**: 30-day grace period if no internet
- ✅ **GDPR compliant**: Minimal data collection
- ✅ **Transparent pricing**: No hidden fees
- ✅ **Cancel anytime**: No lock-in
- ✅ **Data preservation**: Downgrade never deletes user data

---

## STRATEGIC DECISION POINT: Choose Your Path 🔀

**After completing GDPR compliance, AI Act compliance, and basic PII protection (Phases 1-5), you face a critical architectural decision.**

### Path A: Markdown-First Architecture 📝
- Plaintext philosophy
- Git version control
- Maximum AI accessibility
- **→ See PHASE_6A.md**

### Path B: Microsoft Word Integration 📄
- Familiar workflows
- Word Add-in with local AI
- Agent-based automation
- **→ See PHASE_6B.md**

### Path C: Hybrid Approach
- Internal work in Markdown
- Client deliverables in Word
- Best of both worlds
