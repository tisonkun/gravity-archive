/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2022-present tison <wander4096@gmail.com>
 */
package org.tisonkun.gravity.model.github.event;

import lombok.Builder;
import lombok.Data;
import lombok.extern.jackson.Jacksonized;
import org.tisonkun.gravity.model.github.model.Issue;
import org.tisonkun.gravity.model.github.model.IssueComment;
import org.tisonkun.gravity.model.github.model.IssueCommentChanges;

@Jacksonized
@Builder
@Data
public class IssueCommentEvent implements Event {
    private final String action;
    private final IssueCommentChanges changes;
    private final Issue issue;
    private final IssueComment comment;
}
