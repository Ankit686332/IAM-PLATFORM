CREATE TABLE member_roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    membership_id UUID NOT NULL,

    role_id UUID NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(membership_id, role_id),

    CONSTRAINT fk_member_role_membership
        FOREIGN KEY(membership_id)
        REFERENCES memberships(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_member_role_role
        FOREIGN KEY(role_id)
        REFERENCES roles(id)
        ON DELETE CASCADE
);
