use ink_lang::contract;

contract! {
    #![env = ink_env::DefaultEnvironment]
    
    /// A contract for posting, commenting, upvoting, and downvoting location-based reviews.
    struct KalloViewRegistry {
        // Mapping to track if an address has voted on a review
        voted: ink_storage::collections::HashMap<(Hash, AccountId), bool>,
        // Mapping to store reviews by location and author
        reviews: ink_storage::collections::HashMap<Hash, bool>,
    }

    impl KalloViewRegistry {
        /// Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                voted: Default::default(),
                reviews: Default::default(),
            }
        }

        /// Post a review for a location.
        #[ink(message)]
        pub fn post_review(&mut self, location_id: Hash, review_id: Hash) {
            let review_hash = self.env().caller();
            if self.reviews.contains_key(&review_hash) {
                self.env().emit_event(AlreadyReviewed {});
                return;
            }

            self.reviews.insert(review_hash, true);

            self.env().emit_event(ReviewPosted {
                location_id,
                review_id,
                author: self.env().caller(),
            });
        }

        /// Comment on a review.
        #[ink(message)]
        pub fn comment_review(&self, location_id: Hash, review_id: Hash, comment_id: Hash) {
            // There is no limit on comments
            // So we don't have any validations
            self.env().emit_event(ReviewCommented {
                location_id,
                review_id,
                comment_id,
                author: self.env().caller(),
            });
        }

        /// Upvote a review.
        #[ink(message)]
        pub fn upvote_review(&mut self, location_id: Hash, review_id: Hash) {
            let voter = self.env().caller();
            self._assure_user_has_not_voted(review_id, voter);

            self.env().emit_event(ReviewUpvoted {
                location_id,
                review_id,
                voter,
            });
        }

        /// Downvote a review.
        #[ink(message)]
        pub fn downvote_review(&mut self, location_id: Hash, review_id: Hash) {
            let voter = self.env().caller();
            self._assure_user_has_not_voted(review_id, voter);

            self.env().emit_event(ReviewDownvoted {
                location_id,
                review_id,
                voter,
            });
        }

        fn _assure_user_has_not_voted(&mut self, review_id: Hash, voter: AccountId) {
            if self.voted.get(&(review_id, voter)).copied().unwrap_or(false) {
                self.env().emit_event(AlreadyVoted {});
                self.env().revert();
            }

            self.voted.insert((review_id, voter), true);
        }
    }
    
    #[ink(event)]
    struct ReviewPosted {
        location_id: Hash,
        review_id: Hash,
        author: AccountId,
    }

    #[ink(event)]
    struct ReviewUpvoted {
        location_id: Hash,
        review_id: Hash,
        voter: AccountId,
    }

    #[ink(event)]
    struct ReviewDownvoted {
        location_id: Hash,
        review_id: Hash,
        voter: AccountId,
    }

    #[ink(event)]
    struct ReviewCommented {
        location_id: Hash,
        review_id: Hash,
        comment_id: Hash,
        author: AccountId,
    }

    #[ink(error)]
    struct AlreadyVoted;

    #[ink(error)]
    struct AlreadyReviewed;
}
