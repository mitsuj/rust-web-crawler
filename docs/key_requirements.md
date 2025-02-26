# Key Requirements

## Functional Requirements

1. **URL Discovery**
- Extract all `href` attributes from HTML
- Normalize URLs (handle relative paths)
- Filter by domain/subdomain

2. **Content Handling**
- Store raw HTML
- Extract metadata (title, headers)
- Detect content type (HTML/PDF/etc.)

3. **Politeness**
- Respect robots.txt directives
- 1-2 second delay between same-domain requests
- Honor `Crawl-Delay` if specified

4. **Resilience**
- Retry failed requests (3 attempts)
- Handle HTTP 429/503 responses
- Timeout after 10 seconds per request

## Non-Functional Requirements
| **Attribute** | **Target**                    | **Rationale**
----------------|-------------------------------|-------------------------------------------
| Throughput    | 100 reqs/sec (single node)    | Avoid overwhelming targets
| Scalability   | Vertical scaling first        | Keep initial design simple
| Latency       | <2s per page (median)         | Balance speed vs politeness
| Durability    | 99.9% data persistence        | Prevent data loss
| Accuracy      | 100% URL deduplication        | Avoid redundant work