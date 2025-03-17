pub enum ApprovalResult {
    Approved(Post),
    Pending(PendingReviewPost),
}
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
    approvals: u32,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: 0,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> ApprovalResult {
        match self.approvals {
            1 => ApprovalResult::Approved(Post {
                content: self.content,
            }),
            _ => ApprovalResult::Pending(PendingReviewPost {
                content: self.content,
                approvals: self.approvals + 1,
            }),
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        if let ApprovalResult::Pending(post) = post.approve() {
            if let ApprovalResult::Approved(post) = post.approve() {
                assert_eq!("I ate a salad for lunch today", post.content());
            } else {
                panic!("Should be in approved state here...")
            }
        }
    }

    #[test]
    fn it_rejects() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.reject();

        let post = post.request_review();

        if let ApprovalResult::Pending(post) = post.approve() {
            if let ApprovalResult::Approved(post) = post.approve() {
                assert_eq!("I ate a salad for lunch today", post.content());
            } else {
                panic!("Should be in approved state here...")
            }
        }
    }
}
