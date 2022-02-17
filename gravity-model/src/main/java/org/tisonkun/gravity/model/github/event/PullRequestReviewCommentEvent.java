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
import org.tisonkun.gravity.model.github.model.PullRequestComment;
import org.tisonkun.gravity.model.github.model.PullRequestReviewComentChanges;

@Jacksonized
@Builder
@Data
public class PullRequestReviewCommentEvent implements Event {
    private final String action;
    @JsonProperty("pull_request")
    private final PullRequest pullRequest;
    private final PullRequestComment comment;
    private final PullRequestReviewComentChanges changes;
}
