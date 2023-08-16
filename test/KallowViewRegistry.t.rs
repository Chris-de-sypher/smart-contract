
// SPDX-License-Identifier: UNLICENSED
use forge_std::Test;
use crate::src::KalloViewRegistry;

pub struct KalloViewRegistryTest {
    registry: KalloViewRegistry,
}

impl Test for KalloViewRegistryTest {
    fn set_up(&mut self) {
        self.registry = KalloViewRegistry::new();
    }

    fn test_post_review(&mut self, location_id: [u8; 32], review_id: [u8; 32]) {
        vm.assume(location_id != [0u8; 32]);
        vm.assume(review_id != [0u8; 32]);
        vm.assume(location_id != review_id);

        vm.expect_emit(true, true, true, true);
        emit! { ReviewPosted(location_id, review_id, address(this)) };
        self.registry.post_review(location_id, review_id);
    }

    fn test_upvote(&mut self, location_id: [u8; 32], review_id: [u8; 32]) {
        vm.assume(location_id != [0u8; 32]);
        vm.assume(review_id != [0u8; 32]);
        vm.assume(location_id != review_id);

        vm.expect_emit(true, true, true, true);
        emit! { ReviewUpvoted(location_id, review_id, address(this)) };
        self.registry.upvote_review(location_id, review_id);
    }

    fn test_downvote(&mut self, location_id: [u8; 32], review_id: [u8; 32]) {
        vm.assume(location_id != [0u8; 32]);
        vm.assume(review_id != [0u8; 32]);
        vm.assume(location_id != review_id);

        vm.expect_emit(true, true, true, true);
        emit! { ReviewDownvoted(location_id, review_id, address(this)) };
        self.registry.downvote_review(location_id, review_id);
    }

    fn test_comment(&mut self, location_id: [u8; 32], review_id: [u8; 32], comment_id: [u8; 32]) {
        vm.assume(location_id != [0u8; 32]);
        vm.assume(review_id != [0u8; 32]);
        vm.assume(comment_id != [0u8; 32]);
        vm.assume(location_id != review_id);
        vm.assume(location_id != comment_id);
        vm.assume(review_id != comment_id);

        vm.expect_emit(true, true, true, true);
        emit! { ReviewCommented(location_id, review_id, comment_id, address(this)) };
        self.registry.comment_review(location_id, review_id, comment_id);
    }
}