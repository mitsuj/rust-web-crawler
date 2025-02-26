# Scope Definition

## In Scope (MVP)

|**Component**          |   **Responsibility**                      |   **Tech Choice**         |
------------------------|-------------------------------------------|----------------------------
| URL Frontier          | Manage crawl queue and deduplication      | Redis
| Fetcher               | HTTP content retrieval                    | Reqwest + Tokio
| Parser                | Extract links and content                 | Scraper + Regex
| Politeness Controller | Rate limiting & `robots.txt` compliance   | Custom + `robots.txt` crate
| Storage               | Store crawled content & URLs              | SQLite + Redis
| Monitoring            | Basic metrics (URLs crawled, errors)      | Prometheus (Optional)


## Out of Scope (For Later)

- Distributed crawling across multiple machines
- JavaScript rendering (headless browsers)
- Advanced content analysis (NLP/ML)
- Authentication/cookie handling
- CAPTCHA solving
- Geographically distributed crawling