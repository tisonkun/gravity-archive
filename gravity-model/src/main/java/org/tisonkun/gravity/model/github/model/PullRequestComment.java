/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2022-present tison <wander4096@gmail.com>
 */

package org.tisonkun.gravity.model.github.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Builder;
import lombok.Data;
import lombok.extern.jackson.Jacksonized;

@Jacksonized
@Builder
@Data
public class PullRequestComment {
    private final long id;
    @JsonProperty("node_id")
    private final String nodeId;
    private final String url;
    @JsonProperty("html_url")
    private final String htmlUrl;
    @JsonProperty("pull_request_review_id")
    private final long pullRequestReviewId;
    @JsonProperty("diff_hunk")
    private final String diffHunk;
    private final String path;
    private final long position;
    @JsonProperty("original_position")
    private final long originalPosition;
    @JsonProperty("commit_id")
    private final String commitId;
    @JsonProperty("original_commit_id")
    private final String originalCommitId;
    private final User user;
    @JsonProperty("created_at")
    private final String createdAt;
    @JsonProperty("updated_at")
    private final String updatedAt;
    private final String body;
    @JsonProperty("author_association")
    private final String authorAssociation;
    @JsonProperty("start_line")
    private final long startLine;
    @JsonProperty("original_start_line")
    private final long originalStartLine;
    @JsonProperty("start_side")
    private final String startSide;
    private final long line;
    @JsonProperty("original_line")
    private final long originalLine;
    private final String side;
    @JsonProperty("in_reply_to_id")
    private final long inReplyToId;
}
