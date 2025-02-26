# Problem Statement

Build a **distributed, high-performance web crawler** that can:
1. **Systematically discover and download** web pages starting from seed URLs.
1. **Extract strucutred data** (text, metadata, links) and discover new pages.
1. **Operate at scale**, processing thousands of pages per minute
1. **Respect website policies**, including ```robots.txt``` and rate limits
1. **Handle failures gracefully**, such as network errors and malformed content.
1. **Store results effeciently** for downstream processing.
