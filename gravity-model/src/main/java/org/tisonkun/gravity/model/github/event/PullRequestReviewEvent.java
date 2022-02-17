/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2022-present tison <wander4096@gmail.com>
 */
package org.tisonkun.gravity.model.github.event;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Builder;
import lombok.Data;
import lombok.extern.jackson.Jacksonized;
import org.tisonkun.gravity.model.github.model.PullRequest;
import org.tisonkun.gravity.model.github.model.PullRequestReview;
import org.tisonkun.gravity.model.github.model.PullRequestReviewChanges;

@Jacksonized
@Builder
@Data
public class PullRequestReviewEvent implements Event {
    private final String action;
    @JsonProperty("pull_request")
    private final PullRequest pullRequest;
    private final PullRequestReview review;
    private final PullRequestReviewChanges changes;
}
