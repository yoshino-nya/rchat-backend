--
-- PostgreSQL database dump
--

\restrict bIh4WhFUBq13CzsnX4gMaWHSBPinjiU3f3OvpEykbiTP0WEmSVY1vooWoc2RdOa

-- Dumped from database version 18.3
-- Dumped by pg_dump version 18.3

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: chat_session_type; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.chat_session_type AS ENUM (
    'private',
    'group'
);


--
-- Name: friend_request_status; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.friend_request_status AS ENUM (
    'accepted',
    'rejected',
    'pending'
);


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: chat_message; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.chat_message (
    id integer NOT NULL,
    content text NOT NULL,
    created_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    session_id uuid,
    sender_id integer
);


--
-- Name: chat_message_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.chat_message_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: chat_message_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.chat_message_id_seq OWNED BY public.chat_message.id;


--
-- Name: chat_session; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.chat_session (
    id integer NOT NULL,
    type public.chat_session_type,
    last_msg integer DEFAULT 0,
    uuid uuid NOT NULL,
    name text,
    avatar uuid
);


--
-- Name: chat_session_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.chat_session_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: chat_session_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.chat_session_id_seq OWNED BY public.chat_session.id;


--
-- Name: chat_session_members; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.chat_session_members (
    id integer NOT NULL,
    session_id uuid,
    user_id integer,
    last_read integer,
    remark text
);


--
-- Name: chat_session_members_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.chat_session_members_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: chat_session_members_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.chat_session_members_id_seq OWNED BY public.chat_session_members.id;


--
-- Name: friend_request; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.friend_request (
    id integer NOT NULL,
    user_from integer NOT NULL,
    user_to integer NOT NULL,
    status public.friend_request_status DEFAULT 'pending'::public.friend_request_status,
    created_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP
);


--
-- Name: friend_request_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.friend_request_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: friend_request_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.friend_request_id_seq OWNED BY public.friend_request.id;


--
-- Name: friendship; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.friendship (
    id integer NOT NULL,
    user_low integer NOT NULL,
    user_high integer NOT NULL,
    created_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT friendship_check CHECK ((user_low < user_high))
);


--
-- Name: friendship_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.friendship_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: friendship_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.friendship_id_seq OWNED BY public.friendship.id;


--
-- Name: user; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public."user" (
    id integer NOT NULL,
    username character varying NOT NULL,
    password character varying NOT NULL,
    avatar uuid
);


--
-- Name: user_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_id_seq OWNED BY public."user".id;


--
-- Name: chat_message id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.chat_message ALTER COLUMN id SET DEFAULT nextval('public.chat_message_id_seq'::regclass);


--
-- Name: chat_session id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.chat_session ALTER COLUMN id SET DEFAULT nextval('public.chat_session_id_seq'::regclass);


--
-- Name: chat_session_members id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.chat_session_members ALTER COLUMN id SET DEFAULT nextval('public.chat_session_members_id_seq'::regclass);


--
-- Name: friend_request id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.friend_request ALTER COLUMN id SET DEFAULT nextval('public.friend_request_id_seq'::regclass);


--
-- Name: friendship id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.friendship ALTER COLUMN id SET DEFAULT nextval('public.friendship_id_seq'::regclass);


--
-- Name: user id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."user" ALTER COLUMN id SET DEFAULT nextval('public.user_id_seq'::regclass);


--
-- Name: chat_message chat_message_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.chat_message
    ADD CONSTRAINT chat_message_pkey PRIMARY KEY (id);


--
-- Name: chat_session_members chat_session_members_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.chat_session_members
    ADD CONSTRAINT chat_session_members_pkey PRIMARY KEY (id);


--
-- Name: chat_session_members chat_session_members_session_id_user_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.chat_session_members
    ADD CONSTRAINT chat_session_members_session_id_user_id_key UNIQUE (session_id, user_id);


--
-- Name: chat_session chat_session_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.chat_session
    ADD CONSTRAINT chat_session_pkey PRIMARY KEY (id);


--
-- Name: chat_session chat_session_uuid_unique; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.chat_session
    ADD CONSTRAINT chat_session_uuid_unique UNIQUE (uuid);


--
-- Name: friend_request friend_request_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.friend_request
    ADD CONSTRAINT friend_request_pkey PRIMARY KEY (id);


--
-- Name: friendship friendship_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.friendship
    ADD CONSTRAINT friendship_pkey PRIMARY KEY (id);


--
-- Name: friendship friendship_user_low_user_high_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.friendship
    ADD CONSTRAINT friendship_user_low_user_high_key UNIQUE (user_low, user_high);


--
-- Name: user user_avatar_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_avatar_key UNIQUE (avatar);


--
-- Name: user user_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (id);


--
-- Name: user user_username_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_username_key UNIQUE (username);


--
-- PostgreSQL database dump complete
--

\unrestrict bIh4WhFUBq13CzsnX4gMaWHSBPinjiU3f3OvpEykbiTP0WEmSVY1vooWoc2RdOa

