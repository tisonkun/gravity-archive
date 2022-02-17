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
public class Milestone {
    private final long id;
    @JsonProperty("node_id")
    private final String nodeId;
    private final String url;
    @JsonProperty("html_url")
    private final String htmlUrl;
    private final long number;
    private final String state;
    private final String title;
    private final String description;
    private final User creator;
    @JsonProperty("open_issues")
    private final long openIssues;
    @JsonProperty("closed_issues")
    private final long closedIssues;
    @JsonProperty("created_at")
    private final String createdAt;
    @JsonProperty("updated_at")
    private final String updatedAt;
    @JsonProperty("closed_at")
    private final String closedAt;
    @JsonProperty("due_on")
    private final String dueOn;
}
