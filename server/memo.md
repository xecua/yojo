## 足りない機能
+ 削除 ← unimplemented!
+ 更新 ← unimplemented!
+ 検索(本文) ← クライアント側でやればよさそう
+ タグ絞り込み ← 同じく


## DB Schema
### Tweet
| Column | Type | Opt |
| -- | -- | -- |
| ID | VARCHAR(36) | UUID, PRIMARY KEY |
| TweetID | VARCHAR(64) | UNIQUE, NOT NULL |
| HTML | VARCHAR(5000) | NOT NULL |
| comment | VARCHAR(10000) | NOT NULL |

### Tag
| Column | Type | Opt |
| -- | -- | -- |
| ID | VARCHAR(36) | UUID, PRIMARY KEY |
| Tag | VARCHAR(255) | NOT NULL |

### Tweets-To-Tags
Many to Many Relationship
| Column | Type | Opt |
| -- | -- | -- |
| ID | VARCHAR(36) | UUID, PRIMARY KEY |
| Tweet.id | VARCHAR(36) | FOREIGN KEY |
| Tag.id | VARCHAR(36) | FOREIGN KEY |
