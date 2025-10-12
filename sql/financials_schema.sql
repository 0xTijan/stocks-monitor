-- ==========================
-- 2. Reports Metadata
-- ==========================
CREATE TABLE reports (
    report_id SERIAL PRIMARY KEY,
    company_id INT REFERENCES companies(company_id) ON DELETE CASCADE,
    period_end DATE NOT NULL,
    period_type VARCHAR(10) CHECK (period_type IN ('Q1','Q2','Q3','Q4','1H','FY')),
    currency VARCHAR(10) DEFAULT 'EUR',
    source_type VARCHAR(20) CHECK (source_type IN ('Slovenia','Croatia','Austria','Other')),
    source_file TEXT
);

-- ==========================
-- 3. Core Financials
-- ==========================
CREATE TABLE financials (
    financial_id SERIAL PRIMARY KEY,
    report_id INT REFERENCES reports(report_id) ON DELETE CASCADE,
    revenue NUMERIC(20,2),
    ebitda NUMERIC(20,2),
    ebit NUMERIC(20,2),
    net_income NUMERIC(20,2),
    earnings_per_share NUMERIC(20,4),
    total_assets NUMERIC(20,2),
    total_equity NUMERIC(20,2),
    roe NUMERIC(10,4), -- in %
    roa NUMERIC(10,4), -- in %
    liabilities_equity_ratio NUMERIC(10,4),
    net_working_capital NUMERIC(20,2),
    net_liabilities NUMERIC(20,2)
);

-- ==========================
-- 4. Valuation Ratios
-- ==========================
CREATE TABLE valuation_ratios (
    valuation_id SERIAL PRIMARY KEY,
    report_id INT REFERENCES reports(report_id) ON DELETE CASCADE,
    p_e NUMERIC(10,4),
    p_b NUMERIC(10,4),
    p_s NUMERIC(10,4),
    ev_s NUMERIC(10,4),
    net_debt_ebitda NUMERIC(10,4),
    dividend_per_share NUMERIC(10,4),
    dividend_yield NUMERIC(10,4)
);
