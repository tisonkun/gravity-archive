/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2022-present tison <wander4096@gmail.com>
 */

package org.tisonkun.gravity.model.github.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;
import lombok.Builder;
import lombok.Data;
import lombok.extern.jackson.Jacksonized;

@Jacksonized
@Builder
@Data
public class PullRequest {
    private final long id;
    @JsonProperty("node_id")
    private final String nodeId;
    private final String url;
    @JsonProperty("html_url")
    private final String htmlUrl;
    @JsonProperty("diff_url")
    private final String diffUrl;
    @JsonProperty("patch_url")
    private final String patchUrl;
    private final long number;
    private final String title;
    @JsonProperty("created_at")
    private final String createdAt;
    @JsonProperty("updated_at")
    private final String updatedAt;
    @JsonProperty("closed_at")
    private final String closedAt;
    @JsonProperty("merged_at")
    private final String mergedAt;
    private final String state;
    private final boolean locked;
    private final User user;
    private final String body;
    @JsonProperty("merge_commit_sha")
    private final String mergeCommitSha;
    private final List<Label> labels;
    private final User assignee;
    private final List<User> assignees;
    private final Milestone milestone;
    private final boolean draft;
    @JsonProperty("requested_reviewers")
    private final List<User> requestedReviewers;
    private final boolean merged;
    private final boolean mergeable;
    @JsonProperty("mergeable_state")
    private final String mergeableState;
    @JsonProperty("merged_by")
    private final User mergedBy;
    private final long comments;
    @JsonProperty("review_comments")
    private final long reviewComments;
    private final long commits;
    private final long additions;
    private final long deletions;
    @JsonProperty("changed_files")
    private final long changedFiles;
    private final PullRequestRef head;
    private final PullRequestRef base;
    @JsonProperty("maintainer_can_modify")
    private final boolean maintainerCanModify;
    @JsonProperty("author_association")
    private final String authorAssociation;
}
